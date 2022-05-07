use std::str::FromStr;
use graphql_client::{GraphQLQuery, Response};
use surf::http::headers::HeaderValue;

pub mod queries;
pub mod commands;

pub struct GraphqlClient {
    client: surf::Client,
}

impl GraphqlClient {
    pub fn new(token: &str) -> anyhow::Result<Self> {
        let config = surf::Config::new()
            .set_base_url(surf::Url::parse("https://api.github.com")?)
            .add_header("Authorization", HeaderValue::from_str(&format!("Bearer {token}")).map_err(|err| anyhow::anyhow!("{err:?}"))?).map_err(|err| anyhow::anyhow!("{err:?}"))?;
        let client = config.try_into()?;

        Ok(Self {
            client
        })
    }

    pub async fn get_reviews(&self, query: String) -> anyhow::Result<Vec<queries::get_reviews::GetReviewsSearchNodesOnPullRequest>> {
        use queries::get_reviews::*;
        let response = self.query::<queries::GetReviews>(Variables {
            query
        }).await?;
        let prs = response.search.nodes
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .filter_map(|node| if let GetReviewsSearchNodes::PullRequest(pr) = node {
                Some(pr)
            }else {
                None
            })
            .collect();

        Ok(prs)
    }

    pub async fn get_review_file_summaries(&self, owner: String, repo: String, pr: i64) -> anyhow::Result<Vec<queries::get_review_file_summaries::GetReviewFileSummariesRepositoryPullRequestFilesNodes>> {
        let response = self.query::<queries::GetReviewFileSummaries>(queries::get_review_file_summaries::Variables {
            owner,
            repo,
            pr,
        }).await?;
        let repository = response.repository.unwrap();
        let pull_request = repository.pull_request.unwrap();
        let file_nodes = pull_request.files.unwrap().nodes.unwrap();

        let file_summaries = file_nodes.into_iter().flatten().collect();

        Ok(file_summaries)
    }

    pub async fn mark_file_viewed_state(&self, pr: String, file_path: String, viewed: bool) -> anyhow::Result<()> {
        if viewed {
            self.query::<commands::MarkFileAsViewed>(commands::mark_file_as_viewed::Variables {
                pr,
                file: file_path
            }).await?;
        }else {
            self.query::<commands::MarkFileAsUnviewed>(commands::mark_file_as_unviewed::Variables {
                pr,
                file: file_path
            }).await?;
        }

        Ok(())
    }

    pub async fn query<Q: GraphQLQuery>(&self, variables: Q::Variables) -> anyhow::Result<Q::ResponseData> {
        let request = Q::build_query(variables);
        let response = self.client.post("/graphql")
            .body_json(&request)
            .map_err(|err| anyhow::anyhow!("{err:?}"))?
            .recv_json::<Response<Q::ResponseData>>()
            .await
            .map_err(|err| anyhow::anyhow!("{err:?}"))?;

        response.data.ok_or_else(|| anyhow::anyhow!("{:?}", response.errors))
    }
}
