use anyhow::{Context, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest;

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/queries/schema.graphql",
    query_path = "src/graphql/queries/intitial_query.graphql"
)]
pub struct InitialQuery;
pub fn run(variables: initial_query::Variables) -> Result<initial_query::ResponseData> {
    // this is the important line
    let request_body = InitialQuery::build_query(variables);

    let client = reqwest::blocking::Client::new();
    let res = client.post("/graphql").json(&request_body).send()?;
    let response_body: Response<initial_query::ResponseData> = res.json()?;
    let data = response_body
        .data
        .with_context(|| "missing response data")?;

    Ok(data)
}
