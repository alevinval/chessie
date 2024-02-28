.PHONY: lint
lint:
	cargo +nightly fmt
	cargo clippy --tests -- -Dclippy::all

.PHONY: test
test:
	cargo test

.PHONY: cover
cover:
	cargo llvm-cov --html
	open target/llvm-cov/html/index.html
