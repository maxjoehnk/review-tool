#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewsRequestDTO {
    pub limit: u32,
    pub query: Option<String>,
    pub sort_by: Option<String>,
    pub project_id: Option<String>,
    pub skip: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewListDTO {
    pub reviews: Vec<ReviewDescriptorDTO>,
    pub has_more: bool,
    pub total_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewDescriptorDTO {
    pub review_id: ReviewIdDTO,
    pub title: String,
    pub description: Option<String>,
    pub participants: Vec<ParticipantInReviewDTO>,
    pub state: ReviewStateEnum,
    pub is_unread: Option<bool>,
    pub is_ready_to_close: Option<bool>,
    #[serde(default)]
    pub branch: Vec<String>,
    #[serde(default)]
    pub issue: Vec<IssueIdDTO>,
    pub is_removed: Option<bool>,
    pub created_at: u64,
    pub created_by: Option<String>,
    pub updated_at: u64,
    pub completion_rate: Option<CompletionRateDTO>,
    pub discussion_counter: Option<SimpleDiscussionCounterDTO>,
    pub deadline: Option<u64>,
    pub is_muted: Option<bool>,
    #[serde(default)]
    pub labels: Vec<LabelDTO>,
    pub merge_from_branch: Option<String>,
    pub merge_to_branch: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewIdDTO {
    pub project_id: String,
    pub review_id: String,
}

impl ToString for ReviewIdDTO {
    fn to_string(&self) -> String {
        format!("{}/{}", self.project_id, self.review_id)
    }
}

impl FromStr for ReviewIdDTO {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.split('/').collect::<Vec<_>>();
        anyhow::ensure!(result.len() == 2, "Invalid review id format");

        Ok(ReviewIdDTO {
            project_id: result[0].to_string(),
            review_id: result[1].to_string(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantInReviewDTO {
    pub user_id: String,
    pub role: RoleInReviewEnum,
    pub state: Option<ParticipantStateEnum>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RoleInReviewEnum {
    Author = 1,
    Reviewer = 2,
    Watcher = 3,
}

impl Display for RoleInReviewEnum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Author => write!(f, "Author"),
            Self::Reviewer => write!(f, "Reviewer"),
            Self::Watcher => write!(f, "Watcher"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ParticipantStateEnum {
    Unread = 1,
    Read = 2,
    Accepted = 3,
    Rejected = 4,
}

impl Display for ParticipantStateEnum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Unread => write!(f, "Unread"),
            Self::Read => write!(f, "Read"),
            Self::Accepted => write!(f, "Accepted"),
            Self::Rejected => write!(f, "Rejected"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueIdDTO {
    pub issue_id: String,
    pub issue_link: Option<String>,
    pub is_created_from_upsource: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ReviewStateEnum {
    Open = 1,
    Closed = 2,
}

impl Display for ReviewStateEnum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionRateDTO {
    pub completed_count: i32,
    pub reviewers_count: i32,
    pub has_concern: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDiscussionCounterDTO {
    pub count: i32,
    pub has_unresolved: bool,
    pub unresolved_count: i32,
    pub resolved_count: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelDTO {
    pub id: Option<String>,
    pub name: String,
    pub color_id: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoRequestDTO {
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoResponseDTO {
    #[serde(default)]
    pub infos: Vec<FullUserInfoDTO>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullUserInfoDTO {
    pub user_id: String,
    pub name: String,
    pub is_resolved: bool,
    pub is_me: bool,
    pub is_online: Option<bool>,
    pub avatar_url: Option<String>,
    pub profile_url: Option<String>,
    pub email: Option<String>,
    pub login: Option<String>,
    pub absent_until: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSummaryDiscussionsRequestDTO {
    pub review_id: ReviewIdDTO,
    pub revisions: Option<RevisionsSetDTO>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionsSetDTO {
    pub revisions: Vec<String>,
    pub select_all: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscussionsInFilesDTO {
    #[serde(default)]
    pub discussions: Vec<DiscussionInFileWithFileDTO>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscussionInFileWithFileDTO {
    pub revision_id: Option<String>,
    pub file_name: String,
    pub discussion_in_file: DiscussionInFileDTO,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscussionInFileDTO {
    pub discussion_id: String,
    pub anchor: AnchorDTO,
    pub origin: Option<AnchorDTO>,
    pub comments: Vec<CommentDTO>,
    // pub read: Option<ReadEnum>,
    pub is_starred: Option<bool>,
    pub issue: Option<String>,
    pub is_resolved: Option<bool>,
    pub sync_result: Option<SyncResultEnum>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnchorDTO {
    pub range: Option<RangeDTO>,
    pub file_id: Option<String>,
    pub revision_id: Option<String>,
    pub inline_in_revision: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeDTO {
    pub start_offset: u32,
    pub end_offset: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentDTO {
    pub discussion_id: String,
    pub comment_id: String,
    pub text: String,
    pub author_id: String,
    pub date: u64,
    pub parent_id: Option<String>,
    pub is_editable: bool,
    pub markup_type: Option<String>,
    pub is_synchronized: bool,
    pub sync_result: Option<SyncResultEnum>,
    pub is_read: bool,
    #[serde(default)]
    pub reactions: Vec<ReactionDTO>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SyncResultEnum {
    /// Revision is reachable via heads
    Reachable = 1,
    /// Unknown revision
    Unknown = 2,
    /// Revision used to be reachable and can become 'Unknown' shortly
    NotReachable = 3,
}

impl Display for SyncResultEnum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Reachable => write!(f, "Reachable"),
            Self::Unknown => write!(f, "Unknown"),
            Self::NotReachable => write!(f, "NotReachable"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactionDTO {
    /// ID of the reaction
    pub id: String,
    /// List of Upsource user IDs
    #[serde(default)]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSummaryChangesRequestDTO {
    pub review_id: ReviewIdDTO,
    pub revisions: Option<RevisionsSetDTO>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewSummaryChangesResponseDTO {
    /// See RevisionsDiffDTO parameters
    pub diff: Option<RevisionsDiffDTO>,
    /// Review annotation message, e.g. "Some revisions in review are not indexed yet", "Review has too many files"
    pub annotation: Option<String>,
    /// The list of files that were omitted in a review according to project settings
    #[serde(default)]
    pub ignored_files: Vec<String>,
    /// See FileDiffSummaryDTO parameters
    #[serde(default)]
    pub file_diff_summary: Vec<FileDiffSummaryDTO>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionsDiffDTO {
    #[serde(default)]
    pub diff: Vec<RevisionDiffItemDTO>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionDiffItemDTO {
    pub project_id: String,
    pub diff_type: DiffTypeEnum,
    pub new_file: FileInRevisionDTO,
    pub old_file: Option<FileInRevisionDTO>,
    pub file_icon: Option<String>,
    pub is_read: bool,
    pub conflict_type: Option<ConflictTypeEnum>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DiffTypeEnum {
    Added = 1,
    Removed = 2,
    Modified = 3,
    Commented = 4,
}

impl Display for DiffTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Added => write!(f, "Added"),
            Self::Removed => write!(f, "Removed"),
            Self::Modified => write!(f, "Modified"),
            Self::Commented => write!(f, "Commented"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ConflictTypeEnum {
    NoConflict = 1,
    CanBeResolved = 2,
    CanNotBeResolved = 3,
}

impl Display for ConflictTypeEnum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::NoConflict => write!(f, "NoConflict"),
            Self::CanBeResolved => write!(f, "CanBeResolved"),
            Self::CanNotBeResolved => write!(f, "CanNotBeResolved"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDiffSummaryDTO {
    pub file: FileInRevisionDTO,
    pub added_lines: u32,
    pub removed_lines: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInRevisionDTO {
    pub project_id: String,
    pub revision_id: String,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInReviewDiffRequestDTO {
    pub file: FileInReviewDTO,
    pub ignore_whitespace: bool,
    pub revisions: Option<RevisionsSetDTO>,
    pub show_unrelated_changes: Option<bool>,
    pub context_lines: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInReviewDTO {
    pub review_id: ReviewIdDTO,
    pub file: FileInRevisionDTO,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInlineDiffResponseDTO {
    pub is_identical: bool,
    pub text: String,
    pub old_file: Option<FileInRevisionDTO>,
    pub new_file: Option<FileInRevisionDTO>,
    pub content_type: FileContentTypeDTO,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileContentTypeDTO {
    pub is_text: bool,
    pub is_directory: bool,
    pub is_generated: bool,
    pub can_download: bool,
    pub file_type: String,
    #[serde(default)]
    pub added_lines: Vec<u32>,
    #[serde(default)]
    pub removed_lines: Vec<u32>,
    #[serde(default)]
    pub modified_lines: Vec<u32>,
    #[serde(default)]
    pub collapsed_lines: Vec<RangeDTO>,
    #[serde(default)]
    pub added_ranges: Vec<RangeDTO>,
    #[serde(default)]
    pub removed_ranges: Vec<RangeDTO>,
    pub is_syntax_supported: Option<bool>,
    #[serde(default)]
    pub syntax_markup: Vec<TextMarkupDTO>,
    #[serde(default)]
    pub diff_to_old_document: Vec<RangeMappingDTO>,
    #[serde(default)]
    pub diff_to_new_document: Vec<RangeMappingDTO>,
    #[serde(default)]
    pub old_line_numbers: Vec<u32>,
    #[serde(default)]
    pub new_line_numbers: Vec<u32>,
    #[serde(default)]
    pub annotation: Vec<FileAnnotationSectionDTO>,
    pub has_unrelated_changes: Option<bool>,
    pub conflict_type: Option<ConflictTypeEnum>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMarkupDTO {
    pub range: RangeDTO,
    pub text_attribute: TextAttributeDTO,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextAttributeDTO {
    #[serde(rename = "fgColor")]
    pub foreground_color: Option<String>,
    #[serde(rename = "bgColor")]
    pub background_color: Option<String>,
    pub font_style: Option<String>,
    pub effect_style: Option<String>,
    pub effect_color: Option<String>,
    pub error_stripe_color: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeMappingDTO {
    pub from: RangeDTO,
    pub to: RangeDTO,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileAnnotationSectionDTO {
    pub start_line: u32,
    pub line_count: u32,
    pub revision: RevisionInfoDTO,
    pub file_path: String,
    pub prior_change_annotation: Option<Box<FileAnnotationSectionDTO>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionInfoDTO {
    project_id: String,
    revision_id: String,
    revision_date: u64,
    effective_revision_date: u64,
    revision_commit_message: String,
    state: RevisionStateEnum,
    vcs_revision_id: String,
    short_revision_id: String,
    author_id: String,
    reachability: RevisionReachabilityEnum,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    branch_head_label: Vec<String>,
    #[serde(default)]
    parent_revision_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RevisionStateEnum {
    None = 1,
    Found = 2,
    Imported = 3,
}

impl Display for RevisionStateEnum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Found => write!(f, "Found"),
            Self::Imported => write!(f, "Imported"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RevisionReachabilityEnum {
    Reachable = 1,
    Unknown = 2,
    NotReachable = 3,
}

impl Display for RevisionReachabilityEnum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Reachable => write!(f, "Reachable"),
            Self::Unknown => write!(f, "Unknown"),
            Self::NotReachable => write!(f, "NotReachable"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInReviewReadStatusRequestDTO {
    pub review_id: ReviewIdDTO,
    pub file: String,
    pub revisions: RevisionsSetDTO,
    pub mark_as_unread: Option<bool>,
}
