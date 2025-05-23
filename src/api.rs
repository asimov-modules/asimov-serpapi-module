// This is free and unencumbered software released into the public domain.

mod bing;
pub use bing::*;

mod duckduckgo;
pub use duckduckgo::*;

mod google;
pub use google::*;

pub use asimov_module::secrecy::{ExposeSecret, SecretString};

use asimov_module::prelude::{Box, Result, String};
use core::error::Error;

#[derive(Debug)]
pub struct SerpApi {
    pub(crate) api_key: SecretString,
}

impl SerpApi {
    pub fn new(api_key: SecretString) -> Self {
        Self { api_key }
    }

    /// See: https://serpapi.com/bing-search-api
    pub fn search_bing(&self, request: &BingSearchRequest) -> Result<String, Box<dyn Error>> {
        let mut response = ureq::get("https://serpapi.com/search")
            .query("engine", "bing")
            .query("q", &request.q)
            .query("api_key", self.api_key.expose_secret())
            .call()?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }

    /// See: https://serpapi.com/duckduckgo-search-api
    pub fn search_duckduckgo(
        &self,
        request: &DuckDuckGoSearchRequest,
    ) -> Result<String, Box<dyn Error>> {
        let mut response = ureq::get("https://serpapi.com/search")
            .query("engine", "duckduckgo")
            .query("q", &request.q)
            .query("kl", request.kl.as_deref().unwrap_or_default())
            .query("api_key", self.api_key.expose_secret())
            .call()?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }

    /// See: https://serpapi.com/search-api
    pub fn search_google(&self, request: &GoogleSearchRequest) -> Result<String, Box<dyn Error>> {
        let mut response = ureq::get("https://serpapi.com/search")
            .query("engine", "google")
            .query("q", &request.q)
            .query("api_key", self.api_key.expose_secret())
            .call()?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }
}
