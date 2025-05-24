// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "std")]
extern crate std;

pub mod api;
pub mod jq;

mod engine;
mod engines;

use engine::Engine;

pub fn find_engine_for(url: impl AsRef<str>) -> Option<&'static Engine> {
    let url = url.as_ref();
    for (url_pattern, engine) in engines::URL_PREFIX_TO_ENGINE.iter().rev() {
        if url.starts_with(url_pattern) {
            return Some(engine);
        }
    }
    None // not found
}
