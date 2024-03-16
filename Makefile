.PHONY: all format clippy test check clippy-fix run test-verbose

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
check: clippy test format
	@echo "All checks passed" 

run:
	@echo "Running the program"
	cargo run
