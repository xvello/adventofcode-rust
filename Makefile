y%:
	RUST_LOG=debug cargo test --lib $@

answers:
	RUST_LOG=info cargo test --lib 2>&1 | grep "Answers" | awk -F'> ' '{print $$NF}' | sort

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace -- -D warnings

test:
	RUST_LOG=info cargo test --lib

all: test fmt clippy

.PHONY: fmt clippy test all answers
.DEFAULT_GOAL := all
