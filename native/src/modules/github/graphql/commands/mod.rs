use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/modules/github/graphql/schema.graphql",
    query_path = "src/modules/github/graphql/commands/mark_file_viewed_state.graphql",
    response_derives = "Debug, PartialEq",
)]
pub struct MarkFileAsViewed;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/modules/github/graphql/schema.graphql",
    query_path = "src/modules/github/graphql/commands/mark_file_viewed_state.graphql",
    response_derives = "Debug, PartialEq",
)]
pub struct MarkFileAsUnviewed;
