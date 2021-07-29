use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use reqwest;

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/queries.graphql"
)]

pub struct InitialQuery;

async fn run_initial_query(variables: initial_query::Variables) -> Result<()> {
    // this is the important line
    let request_body = InitialQuery::build_query(variables);

    let client = reqwest::Client::new();
    let res = client.post("/graphql").json(&request_body).send().await?;
    let _response_body: Response<initial_query::ResponseData> = res.json().await?;
    Ok(())
}
