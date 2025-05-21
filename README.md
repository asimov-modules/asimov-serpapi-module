# ASIMOV SerpApi Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/asimov-serpapi-module)](https://crates.io/crates/asimov-serpapi-module)

[ASIMOV] module for data import powered by the [SerpApi] search data platform.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation from Source Code

```bash
cargo install asimov-serpapi-module
```

## 👉 Examples

```console
$ export SERPAPI_KEY="..."

$ asimov-serpapi-importer https://www.linkedin.com/in/arto/
```

## ⚙ Configuration

### Environment Variables

- `SERPAPI_KEY`: (required) the [SerpApi API key] to use

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
[SerpApi]: https://serpapi.com
[SerpApi API key]: https://serpapi-python.readthedocs.io/en/latest/#serpapi.Client
