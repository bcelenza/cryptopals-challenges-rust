RUSTFLAGS=-C target-cpu=native

.PHONY: test
test:
	RUSTFLAGS="$(RUSTFLAGS)" RUST_BACKTRACE=1 cargo test -- --nocapture

.PHONY: bench
bench:
	RUSTFLAGS="$(RUSTFLAGS)" cargo bench