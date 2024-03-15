.PHONY: all format clippy test check clippy-fix run

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
check: clippy test format
	@echo "All checks passed" 

run:
	@echo "Running the program"
	cargo run
