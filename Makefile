RUSTFLAGS=-A dead_code -C target-cpu=native

.PHONY: test
test:
	RUSTFLAGS="$(RUSTFLAGS)" cargo test -- --nocapture

.PHONY: bench
bench:
	RUSTFLAGS="$(RUSTFLAGS)" cargo bench