.PHONY: test
test:
	RUSTFLAGS="-A dead_code -C target-cpu=native" cargo test -- --nocapture