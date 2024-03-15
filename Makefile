.PHONY: all format clippy test check

format:
	@echo "Formatting code"
	cargo fmt

clippy:
	@echo "Running clippy"
	cargo clippy

test:
	@echo "Running tests"
	cargo test
check: clippy test format
	@echo "All checks passed"
