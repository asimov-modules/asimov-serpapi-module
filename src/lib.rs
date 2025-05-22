// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

pub mod api;

mod engine;
mod engines;

use engine::Engine;

pub fn find_engine_for(url: impl AsRef<str>) -> Option<&'static Engine> {
    let url = url.as_ref();
    for (url_prefix, dataset) in engines::URL_PREFIX_TO_ENGINE.iter().rev() {
        if url.starts_with(url_prefix) {
            return Some(dataset);
        }
    }
    None // not found
}
