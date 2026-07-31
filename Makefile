CARGO = cargo
JQ = jq
READMER = readmer

all: README.md

README.md: .config/readmer/README.md.liquid
	$(READMER) render $< > $@

tests/fixtures/bing.jsonld: tests/fixtures/bing.json src/jq/search.jq
	$(JQ) -f src/jq/search.jq < $< > $@

tests/fixtures/duckduckgo.jsonld: tests/fixtures/duckduckgo.json src/jq/search.jq
	$(JQ) -f src/jq/search.jq < $< > $@

tests/fixtures/google.jsonld: tests/fixtures/google.json src/jq/search.jq
	$(JQ) -f src/jq/search.jq < $< > $@

build: Cargo.toml
	$(CARGO) build --release

check: Cargo.toml tests/fixtures/bing.jsonld tests/fixtures/duckduckgo.jsonld tests/fixtures/google.jsonld
	$(CARGO) test -- --nocapture

clean: Cargo.toml
	$(CARGO) clean
	rm -rf *~ target

maintainer-clean: Cargo.toml
	rm -f Cargo.lock

.PHONY: all check clean maintainer-clean
.SECONDARY:
.SUFFIXES:
