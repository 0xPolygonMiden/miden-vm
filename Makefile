FEATURES_INTERNALS=--features internals
FEATURES_CONCURRENT_EXEC=--features concurrent,executable
FEATURES_GRAVITON_EXEC=--features concurrent,executable,sve
FEATURES_METAL_EXEC=--features concurrent,executable,metal
PROFILE_OPTIMIZED=--profile optimized
PROFILE_TEST=--profile test-release

bench:
	cargo bench $(PROFILE_OPTIMIZED)

exec:
	cargo build $(PROFILE_OPTIMIZED) $(FEATURES_CONCURRENT_EXEC)

exec-metal:
	cargo build $(PROFILE_OPTIMIZED) $(FEATURES_METAL_EXEC)

exec-graviton:
	RUSTFLAGS="-C target-cpu=native" cargo build $(PROFILE_OPTIMIZED) $(FEATURES_GRAVITON_EXEC)

test:
	cargo test $(PROFILE_TEST) $(FEATURES_INTERNALS)
