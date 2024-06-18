.DEFAULT_GOAL := help

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# -- variables --------------------------------------------------------------------------------------

WARNINGS=RUSTDOCFLAGS="-D warnings"
DEBUG_ASSERTIONS=RUSTFLAGS="-C debug-assertions"
FEATURES_CONCURRENT_EXEC=--features concurrent,executable
FEATURES_LOG_TREE=--features concurrent,executable,tracing-forest
FEATURES_METAL_EXEC=--features concurrent,executable,metal

# -- linting --------------------------------------------------------------------------------------

.PHONY: clippy
clippy: ## Runs Clippy with configs
	cargo +nightly clippy --workspace --all-targets --all-features -- -D warnings


.PHONY: fix
fix: ## Runs Fix with configs
	cargo +nightly fix --allow-staged --allow-dirty --all-targets --all-features


.PHONY: format
format: ## Runs Format using nightly toolchain
	cargo +nightly fmt --all


.PHONY: format-check
format-check: ## Runs Format using nightly toolchain but only in check mode
	cargo +nightly fmt --all --check


.PHONY: lint
lint: format fix clippy ## Runs all linting tasks at once (Clippy, fixing, formatting)

# --- testing -------------------------------------------------------------------------------------

.PHONY: test
test: ## Runs all tests
	$(DEBUG_ASSERTIONS) cargo nextest run --cargo-profile test-release --features internals

# --- docs ----------------------------------------------------------------------------------------

.PHONY: doc
doc: ## Generates & checks documentation
	$(WARNINGS) cargo doc --all-features --keep-going --release

# --- building ------------------------------------------------------------------------------------

.PHONY: build
build: ## Builds VM with optimized profile and features
	cargo build --profile optimized $(FEATURES_CONCURRENT_EXEC)

.PHONY: build-single
build-single: ## Builds VM in single-threaded mode
	cargo build --profile optimized --features executable

.PHONY: build-release
build-release: ## Builds VM with release mode and no optimizations
	cargo build --release $(FEATURES_CONCURRENT_EXEC)

.PHONY: build-metal
build-metal: ## Builds VM for metal
	cargo build --profile optimized $(FEATURES_METAL_EXEC)

.PHONY: build-avx2
build-avx2: ## Builds VM for avx2
	RUSTFLAGS="-C target-feature=+avx2" cargo build --profile optimized $(FEATURES_CONCURRENT_EXEC)

.PHONY: build-sve
build-sve: ## Builds VM for sve
	RUSTFLAGS="-C target-feature=+sve" cargo build --profile optimized $(FEATURES_CONCURRENT_EXEC)

.PHONY: build-info
build-info: ## Builds VM with log tree
	cargo build --profile optimized $(FEATURES_LOG_TREE)

# --- benchmarking --------------------------------------------------------------------------------

.PHONY: bench
bench: ## Runs VM benchmarks
	cargo bench --profile optimized
