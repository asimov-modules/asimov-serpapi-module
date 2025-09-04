// This is free and unencumbered software released into the public domain.

use crate::engine::Engine;

pub static URL_PREFIX_TO_ENGINE: [(&str, Engine); 3] = [
    (
        "https://duckduckgo.com/?q=",
        Engine {
            id: "duckduckgo",
            brand: "DuckDuckGo",
            product: "Search",
        },
    ),
    (
        "https://bing.com/search?q=",
        Engine {
            id: "bing",
            brand: "Bing",
            product: "Search",
        },
    ),
    (
        "https://google.com/search?q=",
        Engine {
            id: "google",
            brand: "Google",
            product: "Search",
        },
    ),
];
