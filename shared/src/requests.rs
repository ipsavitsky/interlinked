use std::num::ParseIntError;

use reqwest::Client;
use serde::Serialize;
use thiserror::Error;
use url::Url;

use crate::new_object_schemes::RecordPayload;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Error in request: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Error parsing response: {0}")]
    ParsingInt(#[from] ParseIntError),
    #[error("Error parsing backend url: {0}")]
    ParsingUrl(#[from] url::ParseError),
}

pub async fn fetch_difficulty(backend_url: &Url) -> Result<usize, RequestError> {
    Ok(Client::new()
        .get(backend_url.join("/api/difficulty")?)
        .send()
        .await?
        .text()
        .await?
        .parse()?)
}

pub async fn create_record<T: RecordPayload + Serialize>(
    backend_url: &Url,
    payload: &T,
) -> Result<String, RequestError> {
    let record = payload.with_challenge(payload.challenge().to_string());
    let post_url = backend_url.join(record.record_type())?;
    Ok(Client::new()
        .post(post_url.as_str())
        .json(&record)
        .send()
        .await?
        .text()
        .await?)
}
