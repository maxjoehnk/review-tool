use std::collections::HashMap;
use std::str::FromStr;

use async_compat::CompatExt;
use itertools::Itertools;
use octorust::auth::Credentials;
use octorust::types::{Order, PrivateUser, PublicUser, PullRequestReviewComment, Sort};
use octorust::Client;

use crate::models::*;
use crate::util::split_file_name;
use crate::ReviewModule;

mod graphql;

pub struct GithubModule {
    client: Client,
    graphql_client: graphql::GraphqlClient,
    query: String,
}

impl GithubModule {
    pub fn new(token: String, query: String) -> anyhow::Result<Self> {
        let graphql_client = graphql::GraphqlClient::new(&token)?;
        let client = Client::new("review-tool", Credentials::Token(token))?;

        Ok(Self {
            client,
            graphql_client,
            query,
        })
    }

    async fn get_reviews(&self) -> anyhow::Result<Vec<Review>> {
        use self::graphql::queries::get_reviews::*;
        let prs = self.graphql_client.get_reviews(self.query.clone()).await?;

        let user_ids = prs
            .iter()
            .flat_map(|pr| {
                pr.reviews
                    .as_ref()
                    .and_then(|reviews| reviews.nodes.clone())
                    .unwrap_or_default()
                    .into_iter()
                    .flatten()
                    .flat_map(|review| review.author.as_ref().map(|user| user.login.clone()))
                    .chain(pr.author.iter().map(|user| user.login.clone()))
            })
            .unique();

        let user_cache = self.get_user_info_cache(user_ids).await?;

        let reviews = prs
            .into_iter()
            .map(|pr| {
                let id = ReviewId {
                    node_id: pr.id,
                    id: pr.number,
                    owner: pr.repository.owner.login,
                    repo: pr.repository.name,
                };
                Review {
                    id: id.to_string(),
                    title: pr.title,
                    state: match pr.review_decision {
                        Some(PullRequestReviewDecision::APPROVED) => ReviewState::Approved,
                        Some(PullRequestReviewDecision::CHANGES_REQUESTED) => ReviewState::Rejected,
                        _ => ReviewState::Pending,
                    },
                    open: pr.closed_at.is_none(),
                    branch_name: pr.base_ref.unwrap().name,
                    reviewers: pr
                        .reviews
                        .and_then(|reviews| reviews.nodes)
                        .unwrap_or_default()
                        .into_iter()
                        .flatten()
                        .flat_map(|review| review.author)
                        .unique_by(|review| review.login.clone())
                        .map(|user| user_cache.get_user(&user.login))
                        .collect(),
                    authors: pr
                        .author
                        .into_iter()
                        .map(|user| user_cache.get_user(&user.login))
                        .collect(),
                }
            })
            .collect();

        Ok(reviews)
    }

    async fn get_review_discussions(
        &self,
        review_id: ReviewId,
    ) -> anyhow::Result<Vec<ReviewDiscussion>> {
        let mut comments = self
            .client
            .pulls()
            .list_all_review_comments(
                &review_id.owner,
                &review_id.repo,
                review_id.id,
                Sort::Created,
                Order::Asc,
                None,
            )
            .await?;

        let user_ids = comments
            .iter()
            .filter_map(|comment| comment.user.clone())
            .map(|user| user.login);
        let user_cache = self.get_user_info_cache(user_ids).await?;

        let mut grouped_discussions =
            HashMap::<i64, (PullRequestReviewComment, Vec<PullRequestReviewComment>)>::new();

        comments.sort_by_key(|c| c.in_reply_to_id);

        for discussion in comments.into_iter() {
            if discussion.in_reply_to_id == 0 {
                grouped_discussions.insert(discussion.id, (discussion, vec![]));
            } else if let Some((_, comments)) =
                grouped_discussions.get_mut(&discussion.in_reply_to_id)
            {
                comments.push(discussion);
            } else {
                println!("Missing parent discussion for {}", discussion.id);
            }
        }

        let discussions = grouped_discussions
            .into_iter()
            .map(|(id, (parent, mut comments))| {
                comments.sort_by_key(|comment| comment.created_at);
                comments.reverse();
                let (file_path_segments, file_name) = split_file_name(&parent.path);
                ReviewDiscussion {
                    id: id.to_string(),
                    resolved: false,
                    file: Some(ReviewFileDiscussion {
                        file_name,
                        file_path: parent.path.clone(),
                        file_path_segments,
                        revision: Some(parent.commit_id.clone()),
                    }),
                    comments: comments
                        .into_iter()
                        .chain([parent].into_iter())
                        .map(|comment| ReviewComment {
                            id: comment.id.to_string(),
                            user: user_cache.get_user(&comment.user.unwrap().login),
                            timestamp: comment
                                .created_at
                                .map(|time| time.timestamp_millis() as u64)
                                .unwrap_or_default(),
                            text: comment.body,
                        })
                        .collect(),
                }
            })
            .collect();

        Ok(discussions)
    }

    async fn get_review_file_summaries(
        &self,
        review_id: ReviewId,
    ) -> anyhow::Result<Vec<ReviewFileSummary>> {
        let pulls = self.client.pulls();
        let files = pulls.list_files(&review_id.owner, &review_id.repo, review_id.id, 100, 1);
        let gql_files = self.graphql_client.get_review_file_summaries(
            review_id.owner.clone(),
            review_id.repo.clone(),
            review_id.id.clone(),
        );
        let (files, gql_files) = futures::future::try_join(files, gql_files).await?;

        let files = files
            .into_iter()
            .map(|file| {
                let (file_path_segments, file_name) = split_file_name(&file.filename);
                let is_read = gql_files
                    .iter()
                    .find(|gql_file| &gql_file.path == &file.filename)
                    .map(|file| {
                        file.viewer_viewed_state
                            == graphql::queries::get_review_file_summaries::FileViewedState::VIEWED
                    })
                    .unwrap_or_default();
                ReviewFileSummary {
                    file_name,
                    file_path: file.filename,
                    file_path_segments,
                    change_type: match file.status.as_str() {
                        "modified" => ChangeType::Modified,
                        "added" => ChangeType::Added,
                        "renamed" => ChangeType::Modified,
                        "removed" => ChangeType::Removed,
                        _ => ChangeType::Modified,
                    },
                    added_lines: file.additions as u32,
                    removed_lines: file.deletions as u32,
                    is_read,
                    revision_id: file
                        .contents_url
                        .clone()
                        .unwrap()
                        .query_pairs()
                        .find(|(key, _)| key == "ref")
                        .map(|(_, value)| value.to_string())
                        .unwrap(),
                }
            })
            .collect();

        Ok(files)
    }

    async fn get_user_info_cache(
        &self,
        user_ids: impl Iterator<Item = String>,
    ) -> anyhow::Result<UserCache> {
        let user_ids = user_ids.sorted().dedup().collect::<Vec<String>>();

        let users_api = self.client.users();
        let users = user_ids
            .iter()
            .map(|username| users_api.get_by_username(username));
        let users = futures::future::try_join_all(users).await?;
        let users = users
            .into_iter()
            .map(|user| {
                if let Some(user) = user.private_user() {
                    (user.login.clone(), user.into())
                } else if let Some(user) = user.public_user() {
                    (user.login.clone(), user.into())
                } else {
                    unreachable!()
                }
            })
            .collect::<HashMap<_, _>>();

        Ok(UserCache(users))
    }

    async fn get_review_file_changes(
        &self,
        review_id: ReviewId,
        file_path: String,
        revision: String,
    ) -> anyhow::Result<ReviewFileChanges> {
        let content_file = self
            .client
            .repos()
            .get_content_file(&review_id.owner, &review_id.repo, &file_path, &revision)
            .await?;

        if content_file.encoding == "base64" {
            let buffer = content_file
                .content
                .split('\n')
                .map(|part| {
                    let decoded = base64::decode(part)?;

                    Ok(decoded)
                })
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            if let Ok(text) = String::from_utf8(buffer) {
                Ok(ReviewFileChanges { text })
            } else {
                Ok(ReviewFileChanges {
                    text: String::new(),
                })
            }
        } else {
            println!("Unknown encoding: {}", content_file.encoding);
            Ok(ReviewFileChanges {
                text: String::new(),
            })
        }
    }

    async fn mark_file_read(
        &self,
        review_id: ReviewId,
        file_path: String,
        _revision: String,
        read: bool,
    ) -> anyhow::Result<()> {
        self.graphql_client
            .mark_file_viewed_state(review_id.node_id, file_path, read)
            .await?;

        Ok(())
    }
}

impl ReviewModule for GithubModule {
    fn get_reviews(&self) -> anyhow::Result<Vec<Review>> {
        smol::block_on(self.get_reviews().compat())
    }

    fn get_review_discussions(&self, review_id: String) -> anyhow::Result<Vec<ReviewDiscussion>> {
        smol::block_on(self.get_review_discussions(review_id.parse()?).compat())
    }

    fn get_review_file_summaries(
        &self,
        review_id: String,
    ) -> anyhow::Result<Vec<ReviewFileSummary>> {
        smol::block_on(self.get_review_file_summaries(review_id.parse()?).compat())
    }

    fn get_review_file_changes(
        &self,
        review_id: String,
        file_path: String,
        revision: String,
    ) -> anyhow::Result<ReviewFileChanges> {
        smol::block_on(
            self.get_review_file_changes(review_id.parse()?, file_path, revision)
                .compat(),
        )
    }

    fn mark_file_read(
        &self,
        review_id: String,
        file_path: String,
        revision: String,
        read: bool,
    ) -> anyhow::Result<()> {
        smol::block_on(self.mark_file_read(review_id.parse()?, file_path, revision, read))
    }
}

impl From<&PrivateUser> for User {
    fn from(user: &PrivateUser) -> Self {
        Self {
            name: user.name.clone(),
            avatar_url: user.avatar_url.clone().map(|user| user.to_string()),
        }
    }
}

impl From<&PublicUser> for User {
    fn from(user: &PublicUser) -> Self {
        Self {
            name: user.name.clone(),
            avatar_url: user.avatar_url.clone().map(|user| user.to_string()),
        }
    }
}

struct UserCache(HashMap<String, User>);

impl UserCache {
    fn get_user(&self, id: &str) -> User {
        self.0[id].clone()
    }
}

#[derive(Debug, Clone)]
struct ReviewId {
    owner: String,
    repo: String,
    id: i64,
    node_id: String,
}

impl FromStr for ReviewId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.split('/').collect::<Vec<_>>();
        anyhow::ensure!(result.len() == 4, "Invalid review id format");

        Ok(ReviewId {
            owner: result[0].to_string(),
            repo: result[1].to_string(),
            id: result[2].parse()?,
            node_id: result[3].parse()?,
        })
    }
}

impl ToString for ReviewId {
    fn to_string(&self) -> String {
        format!("{}/{}/{}/{}", self.owner, self.repo, self.id, self.node_id)
    }
}
