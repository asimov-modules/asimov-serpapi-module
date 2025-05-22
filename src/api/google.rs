// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use asimov_module::prelude::{FromStr, String};
use serde::Serialize;

/// See: https://serpapi.com/search-api#api-parameters
#[derive(Clone, Debug, Default, Serialize)]
pub struct GoogleSearchRequest {
    /// See: https://serpapi.com/search-api#api-parameters-search-query-q
    pub q: String,
}

impl FromStr for GoogleSearchRequest {
    type Err = url::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        url::Url::parse(input).map(|url| {
            let mut output = Self::default();
            for (k, v) in url.query_pairs() {
                match k.as_ref() {
                    "q" => output.q = v.trim().into(),
                    _ => {}
                }
            }
            output
        })
    }
}
