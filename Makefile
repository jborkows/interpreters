.PHONY: all format clippy test check clippy-fix run test-verbose test-with-memory-tracing

format:
	@echo "Formatting code"
	cargo fmt

clippy:
	@echo "Running clippy"
	cargo clippy
 
clippy-fix:
	@echo "Running clippy"
	cargo clippy --fix
test:
	@echo "Running tests"
	cargo test
test-verbose:
	@echo "Running tests"
	cargo test -- --nocapture
test-with-memory-tracing:
	@echo "Running tests with memory tracing"
	cargo test  --features track-allocation -- --test-threads=1 --nocapture
check: clippy test format
	@echo "All checks passed" 

run:
	@echo "Running the program"
	cargo run

run-verbose:
	@echo "Running the program"
	RUST_BACKTRACE=1 cargo run 
