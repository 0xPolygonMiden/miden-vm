FEATURES_INTERNALS=--features internals
FEATURES_CONCURRENT_EXEC=--features concurrent,executable
PROFILE_OPTIMIZED=--profile optimized
PROFILE_TEST=--profile test-release

bench:
	cargo bench $(PROFILE_OPTIMIZED)

exec:
	cargo build $(PROFILE_OPTIMIZED) $(FEATURES_CONCURRENT_EXEC)

test:
	cargo test $(PROFILE_TEST) $(FEATURES_INTERNALS)
