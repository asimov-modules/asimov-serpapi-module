CARGO = cargo
JQ = jq

all: Cargo.toml
	$(CARGO) build --release

check: Cargo.toml tests/fixtures/bing.jsonld tests/fixtures/duckduckgo.jsonld tests/fixtures/google.jsonld
	$(CARGO) test

clean: Cargo.toml
	@rm -rf *~ target
	$(CARGO) clean

tests/fixtures/bing.jsonld: tests/fixtures/bing.json src/jq/search.jq
	$(JQ) -f src/jq/search.jq < $< > $@

tests/fixtures/duckduckgo.jsonld: tests/fixtures/duckduckgo.json src/jq/search.jq
	$(JQ) -f src/jq/search.jq < $< > $@

tests/fixtures/google.jsonld: tests/fixtures/google.json src/jq/search.jq
	$(JQ) -f src/jq/search.jq < $< > $@

.PHONY: all check clean
.SECONDARY:
.SUFFIXES:
