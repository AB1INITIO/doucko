# The site is built by a dependency-free Rust tool (scripts/), compiled on
# first use. Needs cargo and pandoc.
ABINITIO := cargo run --quiet --release --manifest-path scripts/Cargo.toml --
PORT ?= 8000

.PHONY: build watch serve test clean

# Build the site into _site/ from scratch.
build:
	@$(ABINITIO) build

# Build, then rebuild whatever changes as you edit.
watch:
	@$(ABINITIO) watch

# Watch, and serve the site at http://localhost:$(PORT) (PORT=HOST:PORT for others).
serve:
	@$(ABINITIO) serve $(PORT)

# Test the build tool.
test:
	@cargo test --quiet --manifest-path scripts/Cargo.toml

clean:
	@rm -rf _site
	@echo "Cleaned."
