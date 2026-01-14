.PHONY: test test-unit test-integration test-e2e build build-wasm format lint clean

# Run all tests
test: test-unit test-integration

# Run unit tests only
test-unit:
	cargo test --lib

# Run integration tests
test-integration:
	cargo test --test '*'

# Build for native
build:
	cargo build

# Build for WASM
build-wasm:
	./build-wasm.sh

# Run E2E tests (requires WASM build and npm install)
test-e2e: build-wasm
	cd tests/e2e && npm test

# Update E2E snapshots
test-e2e-update: build-wasm
	cd tests/e2e && npm run test:update

# Install E2E dependencies
e2e-install:
	cd tests/e2e && npm install && npx playwright install chromium

# Format code
format:
	cargo fmt

# Lint code
lint:
	cargo clippy -- -D warnings

# Clean build artifacts
clean:
	cargo clean
	rm -rf dist
	rm -rf tests/e2e/node_modules
	rm -rf tests/e2e/test-results
	rm -rf tests/e2e/playwright-report
