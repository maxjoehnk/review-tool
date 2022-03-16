use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

use crate::api::{
    ChangeType, Review, ReviewComment, ReviewDiscussion, ReviewFileChanges, ReviewFileDiscussion,
    ReviewFileSummary,
};
use crate::models::{ReviewState, User};
use crate::util::split_file_name;
use crate::ReviewModule;

use self::api::UpsourceApi;
use self::messages::*;

mod api;
mod messages;

pub struct UpsourceModule {
    api: UpsourceApi,
}

impl UpsourceModule {
    pub const fn new(url: String, token: String) -> Self {
        Self {
            api: UpsourceApi::new(url, token),
        }
    }

    async fn get_reviews(&self) -> anyhow::Result<Vec<Review>> {
        let reviews = self.api.get_reviews().await?;

        let user_ids = reviews
            .iter()
            .flat_map(|review| review.participants.iter())
            .map(|participant| participant.user_id.clone());

        let users = self.get_user_info_cache(user_ids).await?;

        let reviews = reviews
            .into_iter()
            .map(|review| {
                let (authors, reviewers) = review
                    .participants
                    .into_iter()
                    .partition::<Vec<_>, _>(|user| user.role == RoleInReviewEnum::Author);

                Review {
                    id: review.review_id.to_string(),
                    title: review.title,
                    open: review.state == ReviewStateEnum::Open,
                    state: ReviewState::Pending,
                    branch_name: review.branch.first().cloned().unwrap_or_default(),
                    authors: authors
                        .into_iter()
                        .map(|user| users.get_user(&user.user_id))
                        .collect(),
                    reviewers: reviewers
                        .into_iter()
                        .map(|user| users.get_user(&user.user_id))
                        .collect(),
                }
            })
            .collect();

        Ok(reviews)
    }

    async fn get_review_discussions(
        &self,
        review_id: ReviewIdDTO,
    ) -> anyhow::Result<Vec<ReviewDiscussion>> {
        let discussions = self.api.get_review_summary_discussions(review_id).await?;

        let user_ids = discussions
            .iter()
            .flat_map(|discussion| discussion.discussion_in_file.comments.iter())
            .map(|comment| comment.author_id.clone());

        let users = self.get_user_info_cache(user_ids).await?;

        let discussions = discussions
            .into_iter()
            .map(|discussion| {
                let (file_path_segments, file_name) = split_file_name(&discussion.file_name);
                ReviewDiscussion {
                    id: discussion.discussion_in_file.discussion_id,
                    comments: discussion
                        .discussion_in_file
                        .comments
                        .into_iter()
                        .map(|comment| ReviewComment {
                            id: comment.comment_id,
                            text: comment.text,
                            timestamp: comment.date,
                            user: users.get_user(&comment.author_id),
                        })
                        .collect(),
                    resolved: discussion
                        .discussion_in_file
                        .is_resolved
                        .unwrap_or_default(),
                    file: Some(ReviewFileDiscussion {
                        file_name,
                        file_path_segments,
                        file_path: discussion.file_name,
                        revision: discussion.revision_id,
                    }),
                }
            })
            .collect();

        Ok(discussions)
    }

    async fn get_review_summaries(
        &self,
        review_id: ReviewIdDTO,
    ) -> anyhow::Result<Vec<ReviewFileSummary>> {
        let summary_changes = self.api.get_review_summary_changes(review_id).await?;
        let files = summary_changes
            .file_diff_summary
            .into_iter()
            .map(|summary| {
                let (file_path_segments, file_name) = split_file_name(&summary.file.file_name);
                let (change_type, is_read) = if let Some(diff) =
                    summary_changes.diff.as_ref().and_then(|diff| {
                        diff.diff
                            .iter()
                            .find(|diff| diff.new_file.file_name == summary.file.file_name)
                    }) {
                    let change_type = match diff.diff_type {
                        DiffTypeEnum::Added => ChangeType::Added,
                        DiffTypeEnum::Removed => ChangeType::Removed,
                        _ => ChangeType::Modified,
                    };
                    (change_type, diff.is_read)
                } else {
                    (ChangeType::Modified, false)
                };
                ReviewFileSummary {
                    file_name,
                    file_path_segments,
                    file_path: summary.file.file_name,
                    revision_id: summary.file.revision_id,
                    added_lines: summary.added_lines,
                    removed_lines: summary.removed_lines,
                    is_read,
                    change_type,
                }
            })
            .collect();

        Ok(files)
    }

    async fn get_user_info_cache(
        &self,
        user_ids: impl Iterator<Item = String>,
    ) -> anyhow::Result<UserCache> {
        let user_ids = user_ids.sorted().dedup().collect();

        let users = self.api.get_user_info(user_ids).await?;
        let users = users
            .infos
            .into_iter()
            .map(|user| (user.user_id.clone(), user))
            .collect::<HashMap<_, _>>();

        Ok(UserCache(users))
    }

    async fn get_review_file_changes(
        &self,
        review_id: ReviewIdDTO,
        file_path: String,
        revision: String,
    ) -> anyhow::Result<ReviewFileChanges> {
        let response = self
            .api
            .get_file_in_review_summary_inline_changes(FileInReviewDiffRequestDTO {
                file: FileInReviewDTO {
                    file: FileInRevisionDTO {
                        file_name: file_path,
                        revision_id: revision,
                        project_id: review_id.project_id.clone(),
                    },
                    review_id,
                },
                revisions: Some(RevisionsSetDTO {
                    revisions: vec![],
                    select_all: Some(true),
                }),
                context_lines: None,
                ignore_whitespace: false,
                show_unrelated_changes: None,
            })
            .await?;

        Ok(ReviewFileChanges {
            text: response.text,
        })
    }

    async fn mark_file_read(
        &self,
        review_id: ReviewIdDTO,
        file_path: String,
        revision: String,
        read: bool,
    ) -> anyhow::Result<()> {
        self.api
            .set_file_in_review_read_status(FileInReviewReadStatusRequestDTO {
                review_id,
                file: file_path,
                revisions: RevisionsSetDTO {
                    revisions: vec![revision],
                    select_all: None,
                },
                mark_as_unread: Some(!read),
            })
            .await
    }
}

impl ReviewModule for UpsourceModule {
    fn get_reviews(&self) -> anyhow::Result<Vec<Review>> {
        smol::block_on(self.get_reviews())
    }

    fn get_review_discussions(&self, review_id: String) -> anyhow::Result<Vec<ReviewDiscussion>> {
        smol::block_on(self.get_review_discussions(ReviewIdDTO::from_str(&review_id)?))
    }

    fn get_review_file_summaries(
        &self,
        review_id: String,
    ) -> anyhow::Result<Vec<ReviewFileSummary>> {
        smol::block_on(self.get_review_summaries(ReviewIdDTO::from_str(&review_id)?))
    }

    fn get_review_file_changes(
        &self,
        review_id: String,
        file_path: String,
        revision: String,
    ) -> anyhow::Result<ReviewFileChanges> {
        smol::block_on(self.get_review_file_changes(
            ReviewIdDTO::from_str(&review_id)?,
            file_path,
            revision,
        ))
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

impl From<&FullUserInfoDTO> for User {
    fn from(user: &FullUserInfoDTO) -> Self {
        User {
            name: user.name.clone(),
            avatar_url: user.avatar_url.clone(),
        }
    }
}

struct UserCache(HashMap<String, FullUserInfoDTO>);

impl UserCache {
    fn get_user(&self, id: &str) -> User {
        let user = &self.0[id];

        user.into()
    }
}
