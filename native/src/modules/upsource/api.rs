use super::messages::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub struct UpsourceApi {
    pub url: String,
    pub token: String,
}

impl UpsourceApi {
    pub const fn new(url: String, token: String) -> Self {
        Self { url, token }
    }

    pub async fn get_reviews(&self) -> anyhow::Result<Vec<ReviewDescriptorDTO>> {
        let response: ReviewListDTO = self
            .rpc_request(
                "getReviews",
                ReviewsRequestDTO {
                    limit: 20,
                    // query: Some("state: open".into()),
                    ..Default::default()
                },
            )
            .await?;

        Ok(response.reviews)
    }

    pub async fn get_review_summary_discussions(
        &self,
        review_id: ReviewIdDTO,
    ) -> anyhow::Result<Vec<DiscussionInFileWithFileDTO>> {
        let result: DiscussionsInFilesDTO = self
            .rpc_request(
                "getReviewSummaryDiscussions",
                ReviewSummaryDiscussionsRequestDTO {
                    review_id,
                    revisions: Some(RevisionsSetDTO {
                        revisions: vec![],
                        select_all: Some(true),
                    }),
                },
            )
            .await?;

        Ok(result.discussions)
    }

    pub async fn get_review_summary_changes(
        &self,
        review_id: ReviewIdDTO,
    ) -> anyhow::Result<ReviewSummaryChangesResponseDTO> {
        let result: ReviewSummaryChangesResponseDTO = self
            .rpc_request(
                "getReviewSummaryChanges",
                ReviewSummaryChangesRequestDTO {
                    review_id,
                    revisions: Some(RevisionsSetDTO {
                        revisions: vec![],
                        select_all: Some(true),
                    }),
                },
            )
            .await?;

        Ok(result)
    }

    pub async fn get_user_info(
        &self,
        user_ids: Vec<String>,
    ) -> anyhow::Result<UserInfoResponseDTO> {
        self.rpc_request("getUserInfo", UserInfoRequestDTO { ids: user_ids })
            .await
    }

    pub async fn get_file_in_review_summary_inline_changes(
        &self,
        request: FileInReviewDiffRequestDTO,
    ) -> anyhow::Result<FileInlineDiffResponseDTO> {
        self.rpc_request("getFileInReviewSummaryInlineChanges", request)
            .await
    }

    pub async fn set_file_in_review_read_status(
        &self,
        request: FileInReviewReadStatusRequestDTO,
    ) -> anyhow::Result<()> {
        let _: VoidMessage = self
            .rpc_request("setFileInReviewReadStatus", request)
            .await?;

        Ok(())
    }

    async fn rpc_request<TResponse: DeserializeOwned>(
        &self,
        method: &str,
        body: impl Serialize,
    ) -> anyhow::Result<TResponse> {
        let mut res = surf::post(format!("{}/~rpc/{method}", &self.url))
            .content_type("application/json")
            .header("Authorization", format!("Bearer {}", &self.token))
            .body_json(&body)
            .map_err(|err| anyhow::anyhow!("{err:?}"))?
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;

        #[cfg(debug_assertions)]
        if !res.status().is_success() {
            eprintln!("{:?}", res.status());
            eprintln!(
                "{:?}",
                res.body_string()
                    .await
                    .map_err(|err| anyhow::anyhow!("{:?}", err))?
            );
            anyhow::bail!("Upsource api failure");
        }

        anyhow::ensure!(
            res.status().is_success(),
            "Upsource api returned non success status code"
        );

        let res: ApiResult<TResponse> = res
            .body_json()
            .await
            .map_err(|err| anyhow::anyhow!("{:?}", err))?;

        Ok(res.result)
    }
}

#[derive(Debug, Deserialize)]
struct ApiResult<T> {
    pub result: T,
}

#[derive(Debug, Deserialize)]
struct VoidMessage {}
