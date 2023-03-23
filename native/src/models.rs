#[derive(Debug, Clone)]
pub struct Review {
    pub id: String,
    pub title: String,
    pub branch_name: String,
    pub authors: Vec<User>,
    pub reviewers: Vec<User>,
    pub open: bool,
    pub state: ReviewState,
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ReviewState {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct ReviewDiscussion {
    pub id: String,
    pub comments: Vec<ReviewComment>,
    pub resolved: bool,
    pub file: Option<ReviewFileDiscussion>,
}

#[derive(Debug, Clone)]
pub struct ReviewFileDiscussion {
    pub file_name: String,
    pub file_path: String,
    pub file_path_segments: Vec<String>,
    pub revision: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReviewComment {
    pub id: String,
    pub user: User,
    pub text: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct ReviewFileSummary {
    pub file_name: String,
    pub file_path: String,
    pub file_path_segments: Vec<String>,
    pub revision_id: String,
    pub added_lines: u32,
    pub removed_lines: u32,
    pub change_type: ChangeType,
    pub is_read: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
}

#[derive(Debug, Clone)]
pub struct ReviewFileChanges {
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ProviderSettings {
    pub id: String,
    pub name: String,
    pub module: Box<ProviderModule>,
}

#[derive(Debug, Clone)]
pub enum ProviderModule {
    Upsource(UpsourceProviderSettings),
    Github(GithubProviderSettings),
    Gitlab(GitlabProviderSettings),
}

#[derive(Debug, Clone)]
pub struct UpsourceProviderSettings {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct GithubProviderSettings {
    pub token: String,
    pub query: String,
}

#[derive(Debug, Clone)]
pub struct GitlabProviderSettings {
    pub url: String,
    pub token: String,
}
