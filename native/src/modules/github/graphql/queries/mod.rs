use graphql_client::GraphQLQuery;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/modules/github/graphql/schema.graphql",
    query_path = "src/modules/github/graphql/queries/get_review_file_summaries.graphql",
    response_derives = "Debug, PartialEq",
)]
pub struct GetReviewFileSummaries;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/modules/github/graphql/schema.graphql",
    query_path = "src/modules/github/graphql/queries/get_reviews.graphql",
    response_derives = "Clone, Debug, PartialEq",
)]
pub struct GetReviews;
