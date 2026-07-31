# ASIMOV SerpApi Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-2024%2B-blue)](https://endoflife.date/rust)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-serpapi-module)](https://crates.io/crates/asimov-serpapi-module)
[![Documentation](https://img.shields.io/docsrs/asimov-serpapi-module?label=docs.rs)](https://docs.rs/asimov-serpapi-module)
[![Package on PyPI](https://img.shields.io/pypi/v/asimov-serpapi-module.svg)](https://pypi.org/project/asimov-serpapi-module)
[![Package on RubyGems](https://img.shields.io/gem/v/asimov-serpapi-module.svg)](https://rubygems.org/gems/asimov-serpapi-module)
[![Package on NPM](https://img.shields.io/npm/v/asimov-serpapi-module.svg)](https://npmjs.com/package/asimov-serpapi-module)

**[ASIMOV] module for data import powered by the [SerpApi] search data platform.**

## ✨ Features

- Imports structured data from DuckDuckGo, Google, and Bing search results.
- Collects the raw JSON data via the SerpApi real-time API (requires an API key).
- Constructs a semantic knowledge graph based on the [KNOW] ontology.
- Supports plain JSON output as well as [RDF] output in the form of [JSON-LD].
- Distributed as a standalone static binary with zero runtime dependencies.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with the [ASIMOV CLI]

```bash
asimov module install serpapi -v
```

### Installation from PyPI

```bash
pip install -U asimov-serpapi-module
```

### Installation from RubyGems

```bash
gem install asimov-serpapi-module
```

### Installation from NPM

```bash
npm install -g asimov-serpapi-module
```

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
asimov-serpapi-fetcher https://duckduckgo.com/?q=Isaac+Asimov   # JSON
asimov-serpapi-importer https://duckduckgo.com/?q=Isaac+Asimov  # JSON-LD
```

### Fetching Google Results

```bash
asimov-serpapi-fetcher https://www.google.com/search?q=Isaac+Asimov   # JSON
asimov-serpapi-importer https://www.google.com/search?q=Isaac+Asimov  # JSON-LD
```

### Fetching Bing Results

```bash
asimov-serpapi-fetcher https://www.bing.com/search?q=Isaac+Asimov   # JSON
asimov-serpapi-importer https://www.bing.com/search?q=Isaac+Asimov  # JSON-LD
```

## ⚙ Configuration

### Environment Variables

- `SERPAPI_KEY`: (required) the [SerpApi API key] to use

## 📚 Reference

### Command-Line Interface

- `asimov-serpapi-fetcher`: collects JSON data from the SerpApi real-time API
- `asimov-serpapi-importer`: collects and transforms JSON into JSON-LD

#### `asimov-serpapi-fetcher`

```shellsession
$ asimov-serpapi-fetcher --help
asimov-serpapi-fetcher

Usage: asimov-serpapi-fetcher [OPTIONS] [URLS]...

Arguments:
  [URLS]...

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -o, --output <FORMAT>  The output format
  -h, --help             Print help
```

#### `asimov-serpapi-importer`

```shellsession
$ asimov-serpapi-importer --help
asimov-serpapi-importer

Usage: asimov-serpapi-importer [OPTIONS] [URLS]...

Arguments:
  [URLS]...

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -o, --output <FORMAT>  The output format
  -h, --help             Print help
```

### Supported Engines

Engine  | URL Prefix | JSON | RDF
:------ | :--------- | :--: | :--:
Bing | `https://www.bing.com/search?q=` | ✅ | ✅
DuckDuckGo | `https://duckduckgo.com/?q=` | ✅ | ✅
Google | `https://www.google.com/search?q=` | ✅ | ✅
<img width="100" height="1"/> | <img width="550" height="1"/> | <img width="50" height="1"/> | <img width="50" height="1"/>

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-serpapi-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https%3A%2F%2Fgithub.com%2Fasimov-modules%2Fasimov-serpapi-module&text=ASIMOV%20SerpApi%20Module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https%3A%2F%2Fgithub.com%2Fasimov-modules%2Fasimov-serpapi-module&title=ASIMOV%20SerpApi%20Module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https%3A%2F%2Fgithub.com%2Fasimov-modules%2Fasimov-serpapi-module&t=ASIMOV%20SerpApi%20Module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https%3A%2F%2Fgithub.com%2Fasimov-modules%2Fasimov-serpapi-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fgithub.com%2Fasimov-modules%2Fasimov-serpapi-module)

[ASIMOV]: https://github.com/asimov-platform
[JSON-LD]: https://json-ld.org
[KNOW]: https://github.com/know-ontology
[NPM]: https:/npmjs.org
[Python]: https://python.org
[RDF]: https://github.com/rust-rdf
[Ruby]: https://ruby-lang.org
[Rust]: https://rust-lang.org
[SerpApi]: https://serpapi.com
[SerpApi API key]: https://serpapi-python.readthedocs.io/en/latest/#serpapi.Client
