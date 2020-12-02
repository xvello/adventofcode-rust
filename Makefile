fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace -- -D warnings

test:
	cargo test --workspace

all: test fmt clippy

.PHONY: fmt clippy test all
.DEFAULT_GOAL := all
