.PHONY: test
test:
	RUSTFLAGS="-A dead_code" cargo test -- --nocapture