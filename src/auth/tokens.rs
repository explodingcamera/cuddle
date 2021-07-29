use std::collections::HashMap;

use anyhow::{anyhow, Result};
use reqwest;
use serde_derive::Deserialize;

// This `derive` requires the `serde` dependency.
#[derive(Deserialize)]
struct Resp {
    ok: bool,
    token: String,
}

pub fn get_jwt(cid: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();

    let resp: Resp = client
        .post("https://api.app.code.berlin/cid_refresh")
        .header(reqwest::header::COOKIE, format!("cid={}", cid))
        .send()?
        .json()?;

    if !resp.ok {
        return Err(anyhow!("Invalid Response: Not Ok"));
    }

    Ok(resp.token)
}
