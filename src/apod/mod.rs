// SPDX-License-Identifier: MIT OR Apache-2.0

mod date_serde;
mod params;

#[cfg(test)]
mod test;

use chrono::NaiveDate;
use serde::Deserialize;
use thiserror::Error;
use url::Url;

use crate::{Api, ApiError, StellariaError, StellariaResult};
pub use params::{ApodParams, ApodParamsBuilder};

pub struct ApodApi {
    api_key: String,
    reqwest_client: reqwest::Client,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApodResponse {
    pub copyright: Option<String>,
    pub date: NaiveDate,
    pub explanation: String,
    pub hdurl: Option<Url>,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: Url,
}

#[derive(Deserialize, Debug, Error)]
#[error("http code {code}: {msg}")]
pub struct ApodApiError {
    code: u16,
    msg: String,
    service_version: String,
}

#[derive(Deserialize, Debug, Error)]
#[non_exhaustive]
pub enum ApodError {
    #[error(transparent)]
    ApodApiError(#[from] ApodApiError),
    #[error("invalid parameters: {0}")]
    ApodParamsError(String),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ApiResponse {
    Error(ApodApiError),
    One(Box<ApodResponse>),
    Many(Vec<ApodResponse>),
}

impl ApiResponse {
    fn parse(self) -> Result<Vec<ApodResponse>, ApodError> {
        match self {
            ApiResponse::Error(error) => Err(ApodError::ApodApiError(error)),
            ApiResponse::One(response) => Ok(vec![*response]),
            ApiResponse::Many(responses) => Ok(responses),
        }
    }
}

impl ApodApi {
    pub fn new(api_key: String, reqwest_client: reqwest::Client) -> Self {
        Self {
            api_key,
            reqwest_client,
        }
    }
}

impl Api for ApodApi {
    type Params = ApodParams;
    type Response = Vec<ApodResponse>;

    async fn get(&self, params: Self::Params) -> StellariaResult<Self::Response> {
        let url = format!(
            "https://api.nasa.gov/planetary/apod?api_key={}",
            self.api_key
        );

        let resp = self
            .reqwest_client
            .get(&url)
            .query(&params)
            .send()
            .await
            .map_err(StellariaError::RequestError)?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.map_err(StellariaError::RequestError)?;
            let truncated = text.chars().take(1024).collect::<String>();
            return Err(ApodError::ApodApiError(ApodApiError {
                code: status.as_u16(),
                msg: truncated,
                service_version: "unknown".into(),
            })
            .into());
        }

        let apod_resp = resp
            .json::<ApiResponse>()
            .await
            .map_err(StellariaError::RequestError)?;

        let responses = apod_resp.parse().map_err(crate::ApiError::ApodError)?;

        Ok(responses)
    }
}

impl From<ApodApiError> for StellariaError {
    fn from(err: ApodApiError) -> Self {
        ApodError::ApodApiError(err).into()
    }
}

impl From<ApodError> for StellariaError {
    fn from(err: ApodError) -> Self {
        ApiError::ApodError(err).into()
    }
}
