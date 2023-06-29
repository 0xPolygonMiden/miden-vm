FEATURES_INTERNALS=--features internals
FEATURES_CONCURRENT_EXEC=--features concurrent,executable
FEATURES_METAL_EXEC=--features concurrent,executable,metal
PROFILE_OPTIMIZED=--profile optimized
PROFILE_TEST=--profile test-release

bench:
	cargo bench $(PROFILE_OPTIMIZED)

exec:
	cargo build $(PROFILE_OPTIMIZED) $(FEATURES_CONCURRENT_EXEC)

exec-metal:
	cargo build $(PROFILE_OPTIMIZED) $(FEATURES_METAL_EXEC)

test:
	cargo test $(PROFILE_TEST) $(FEATURES_INTERNALS)
