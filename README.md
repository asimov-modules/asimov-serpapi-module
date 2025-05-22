# ASIMOV SerpApi Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/asimov-serpapi-module)](https://crates.io/crates/asimov-serpapi-module)

[ASIMOV] module for data import powered by the [SerpApi] search data platform.

## ✨ Features

- Imports structured data from DuckDuckGo, Google, and Bing search results.
- Collects the raw JSON data via the SerpApi real-time API (requires an API key).
- Constructs a semantic knowledge graph based on the [KNOW] ontology.
- Supports plain JSON output as well as [RDF] output formats such as JSON-LD,
  Turtle, and N-Triples.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation from Source Code

```bash
cargo install asimov-serpapi-module
```

## 👉 Examples

```bash
export SERPAPI_KEY="..."
```

### Fetching DuckDuckGo Results

```bash
asimov-serpapi-fetcher https://duckduckgo.com/?q=Isaac+Asimov
```

### Fetching Google Results

```bash
asimov-serpapi-fetcher https://www.google.com/search?q=Isaac+Asimov
```

### Fetching Bing Results

```bash
asimov-serpapi-fetcher https://www.bing.com/search?q=Isaac+Asimov
```

## ⚙ Configuration

### Environment Variables

- `SERPAPI_KEY`: (required) the [SerpApi API key] to use

## 📚 Reference

### Installed Binaries

- `asimov-serpapi-fetcher`: collects JSON data from the SerpApi real-time API

### Supported Engines

Engine  | URL Prefix | JSON | RDF
:------ | :--------- | :--: | :--:
Bing | `https://www.bing.com/search?q=Isaac+Asimov` | ✅ | 🚧
DuckDuckGo | `https://duckduckgo.com/?q=Isaac+Asimov` | ✅ | 🚧
Google | `https://www.google.com/search?q=Isaac+Asimov` | ✅ | 🚧
<img width="100" height="1"/> | <img width="550" height="1"/> | <img width="50" height="1"/> | <img width="50" height="1"/>

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-serpapi-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-serpapi-module&text=asimov-serpapi-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-serpapi-module&title=asimov-serpapi-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-serpapi-module&t=asimov-serpapi-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-serpapi-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-serpapi-module)

[ASIMOV]: https://github.com/asimov-platform
[KNOW]: https://github.com/know-ontology
[RDF]: https://github.com/rust-rdf
[SerpApi]: https://serpapi.com
[SerpApi API key]: https://serpapi-python.readthedocs.io/en/latest/#serpapi.Client
