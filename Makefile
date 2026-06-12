SPEC_URL := https://rootly-heroku.s3.amazonaws.com/swagger/v1/swagger.json
SPEC_FILE := spec/swagger.json

.PHONY: fetch-spec generate regenerate build test clean clippy fmt check

# Download and normalize the OpenAPI spec (replace vnd.api+json for progenitor compat)
fetch-spec:
	@echo "Fetching Rootly OpenAPI spec..."
	@mkdir -p spec
	@curl -sL "$(SPEC_URL)" | sed 's|application/vnd.api+json|application/json|g' > "$(SPEC_FILE)"
	@echo "Spec saved to $(SPEC_FILE)"

# Generate Rust client from spec (xtask handles additional sanitization)
generate: fetch-spec
	@echo "Generating Rust client from OpenAPI spec..."
	cargo run -p xtask
	@echo "Done! Generated src/generated.rs"

# Full regeneration from scratch: clean, fetch, generate, build
regenerate: clean generate build
	@echo "Full regeneration complete."

build:
	cargo build

check:
	cargo check

test:
	cargo test --lib --tests

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo fmt --all

clean:
	cargo clean
	rm -f $(SPEC_FILE)
	rm -f src/generated.rs
