test:
	RUSTFLAGS="-C debug-assertions -C overflow-checks -C debuginfo=2" cargo test --release --features internals
