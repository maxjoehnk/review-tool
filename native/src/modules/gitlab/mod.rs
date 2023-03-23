use std::str::FromStr;
use gitlab::{api, Gitlab, MergeRequest, Project, ProjectId, UserBasic, MergeRequestState, Discussion, Note, MergeRequestChanges, RepoDiff};
use gitlab::api::projects::Projects;
use gitlab::api::projects::merge_requests::{self, MergeRequests};
use gitlab::api::projects::merge_requests::discussions::MergeRequestDiscussions;
use gitlab::api::Query;
use url::Url;
use crate::{ChangeType, Review, ReviewComment, ReviewDiscussion, ReviewFileChanges, ReviewFileDiscussion, ReviewFileSummary, ReviewModule, ReviewState, User};
use crate::util::split_file_name;

pub struct GitlabModule {
    client: Gitlab
}

impl GitlabModule {
    pub fn new(url: String, token: String) -> anyhow::Result<Self> {
        let scheme = {
            let url = Url::parse(&url)?;
            url.scheme().to_string()
        };
        let host = url.strip_prefix(&format!("{scheme}://")).unwrap();
        let client = if scheme == "http" {
            Gitlab::new_insecure(host, token)?
        }else {
            Gitlab::new(host, token)?
        };

        Ok(Self {
            client
        })
    }

    fn get_projects(&self) -> anyhow::Result<Vec<Project>> {
        let endpoint = Projects::builder()
            .membership(true)
            .with_merge_requests_enabled(true)
            .build()?;
        let projects = api::paged(endpoint, api::Pagination::All)
            .query(&self.client)?;

        Ok(projects)
    }

    fn get_merge_requests_for_project(&self, project_id: ProjectId) -> anyhow::Result<Vec<MergeRequest>> {
        let endpoint = MergeRequests::builder()
            .project(project_id.value())
            .state(merge_requests::MergeRequestState::Opened)
            .build()?;
        let merge_requests = api::paged(endpoint, api::Pagination::All)
            .query(&self.client)?;

        Ok(merge_requests)
    }

    fn get_merge_request_discussions(&self, review_id: ReviewId) -> anyhow::Result<Vec<Discussion>> {
        let endpoint = MergeRequestDiscussions::builder()
            .project(review_id.project)
            .merge_request(review_id.id)
            .build()?;
        let discussions = api::paged(endpoint, api::Pagination::All)
            .query(&self.client)?;

        Ok(discussions)
    }

    fn get_merge_request_changes(&self, review_id: ReviewId) -> anyhow::Result<MergeRequestChanges> {
        let endpoint = merge_requests::MergeRequestChanges::builder()
            .project(review_id.project)
            .merge_request(review_id.id)
            .build()?;
        let changes = endpoint.query(&self.client)?;

        Ok(changes)
    }

    fn get_review_file_changes(&self, review_id: ReviewId, file_path: String, revision: String) -> anyhow::Result<ReviewFileChanges> {
        let endpoint = gitlab::api::projects::repository::files::FileRaw::builder()
            .project(review_id.project)
            .file_path(&file_path)
            .ref_(&revision)
            .build()?;
        let file = api::raw(endpoint).query(&self.client)?;
        let file = String::from_utf8(file)?;

        Ok(ReviewFileChanges {
            text: file
        })
    }
}

impl ReviewModule for GitlabModule {
    fn get_reviews(&self) -> anyhow::Result<Vec<Review>> {
        let projects = self.get_projects()?;
        let mut reviews = vec![];
        for project in projects {
            let merge_requests = self.get_merge_requests_for_project(project.id)?;
            for merge_request in merge_requests {
                let review = merge_request.into();
                reviews.push(review);
            }
        }

        Ok(reviews)
    }

    fn get_review_discussions(&self, review_id: String) -> anyhow::Result<Vec<ReviewDiscussion>> {
        let review_id = ReviewId::from_str(&review_id)?;
        let discussions = self.get_merge_request_discussions(review_id)?;
        let discussions = discussions.into_iter().map(ReviewDiscussion::from).collect();

        Ok(discussions)
    }

    fn get_review_file_summaries(&self, review_id: String) -> anyhow::Result<Vec<ReviewFileSummary>> {
        let review_id = ReviewId::from_str(&review_id)?;
        let changes = self.get_merge_request_changes(review_id)?;

        let revision = changes.diff_refs
            .and_then(|ref_| ref_.head_sha)
            .map(|sha| sha.value().clone())
            .unwrap_or_default();

        Ok(changes.changes.into_iter()
            .map(|diff| {
                let (file_path_segments, file_name) = split_file_name(&diff.new_path);
                ReviewFileSummary {
                    file_path: diff.new_path,
                    file_name,
                    file_path_segments,
                    change_type: match (diff.deleted_file, diff.new_file) {
                        (true, _) => ChangeType::Removed,
                        (_, true) => ChangeType::Added,
                        _ => ChangeType::Modified,
                    },
                    added_lines: 0,
                    removed_lines: 0,
                    is_read: false,
                    revision_id: revision.clone(),
                }
            })
            .collect())
    }

    fn get_review_file_changes(&self, review_id: String, file_path: String, revision: String) -> anyhow::Result<ReviewFileChanges> {
        let review_id = ReviewId::from_str(&review_id)?;

        self.get_review_file_changes(review_id, file_path, revision)
    }

    fn mark_file_read(&self, review_id: String, file_path: String, revision: String, read: bool) -> anyhow::Result<()> {
        // TODO
        Ok(())
    }
}

impl From<MergeRequest> for Review {
    fn from(mr: MergeRequest) -> Self {
        Self {
            id: ReviewId {
                project: mr.project_id.value(),
                id: mr.iid.value(),
            }.to_string(),
            open: mr.state == MergeRequestState::Opened,
            state: ReviewState::Pending, // TODO
            title: mr.title,
            authors: vec![mr.author.into()],
            branch_name: mr.source_branch,
            reviewers: mr.reviewers.into_iter()
                .flatten()
                .map(User::from)
                .collect(),
        }
    }
}

impl From<Discussion> for ReviewDiscussion {
    fn from(discussion: Discussion) -> Self {
        Self {
            id: discussion.id.value().to_string(),
            file: discussion.notes.first()
                .and_then(|note| note.position.clone())
                .map(|note_position| {
                    let (file_path_segments, file_name) = split_file_name(&note_position.new_path);
                    ReviewFileDiscussion {
                        file_name,
                        file_path: note_position.new_path,
                        file_path_segments,
                        revision: Some(note_position.head_sha.value().to_string()),
                    }
                }),
            resolved: discussion.notes.first().and_then(|note| note.resolved).unwrap_or_default(),
            comments: discussion.notes.into_iter().map(ReviewComment::from).collect(),
        }
    }
}

impl From<Note> for ReviewComment {
    fn from(note: Note) -> Self {
        Self {
            id: note.id.to_string(),
            timestamp: note.created_at.timestamp() as u64,
            text: note.body,
            user: note.author.into(),
        }
    }
}

impl From<UserBasic> for User {
    fn from(user: UserBasic) -> Self {
        Self {
            name: user.name,
            avatar_url: user.avatar_url,
        }
    }
}

impl From<RepoDiff> for ReviewFileSummary {
    fn from(diff: RepoDiff) -> Self {
        let (file_path_segments, file_name) = split_file_name(&diff.new_path);
        Self {
            file_path: diff.new_path,
            file_name,
            file_path_segments,
            change_type: match (diff.deleted_file, diff.new_file) {
                (true, _) => ChangeType::Removed,
                (_, true) => ChangeType::Added,
                _ => ChangeType::Modified,
            },
            added_lines: 0,
            removed_lines: 0,
            is_read: false,
            revision_id: String::new()
        }
    }
}

#[derive(Debug, Clone)]
struct ReviewId {
    project: u64,
    id: u64,
}

impl FromStr for ReviewId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.split('/').collect::<Vec<_>>();
        anyhow::ensure!(result.len() == 2, "Invalid review id format");

        Ok(ReviewId {
            project: result[0].parse()?,
            id: result[1].parse()?,
        })
    }
}

impl ToString for ReviewId {
    fn to_string(&self) -> String {
        format!("{}/{}", self.project, self.id)
    }
}
