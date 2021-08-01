use anyhow::{Context, Result};
use graphql_client::{GraphQLQuery, Response};
use reqwest;

type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/queries/schema.graphql",
    query_path = "src/graphql/queries/stats.graphql"
)]
pub struct Stats;
pub fn run(variables: stats::Variables) -> Result<stats::ResponseData> {
    // this is the important line
    let request_body = Stats::build_query(variables);

    let client = reqwest::blocking::Client::new();
    let res = client.post("/graphql").json(&request_body).send()?;
    let response_body: Response<stats::ResponseData> = res.json()?;
    let data = response_body
        .data
        .with_context(|| "missing response data")?;

    Ok(data)
}
