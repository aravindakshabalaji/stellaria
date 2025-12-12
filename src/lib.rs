// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

pub mod apod;

pub trait Api: Send + Sync {
    type Params;
    type Response;

    fn get(
        &self,
        params: Self::Params,
    ) -> impl std::future::Future<Output = StellariaResult<Self::Response>> + Send;
}

pub struct StellariaClient {
    pub apod: apod::ApodApi,
    pub api_token: String,
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum StellariaError {
    #[error("invalid http request: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    ApiError(#[from] ApiError),
    #[error("error in parsing json: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ApiError {
    #[error(transparent)]
    ApodError(#[from] apod::ApodError),
}

pub type StellariaResult<T> = std::result::Result<T, StellariaError>;

impl StellariaClient {
    pub fn new(api_token: impl Into<String>) -> Self {
        let api_token = api_token.into();
        let reqwest_client = reqwest::Client::new();

        Self {
            api_token: api_token.clone(),
            apod: apod::ApodApi::new(api_token, reqwest_client.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stellaria_client_creation() {
        let _ = dotenv::dotenv();

        let token =
            std::env::var("API_TOKEN").expect("API_TOKEN must be set in .env or environment");
        let client = StellariaClient::new(&token);
        assert_eq!(client.api_token, token);
    }
}
