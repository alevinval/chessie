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

.PHONY: run
run:
	cargo run --release --bin chessie

.PHONY: pregen
pregen:
	cargo run --release --bin chessie-pregen
	make lint

.PHONY: bench
bench:
	cargo run --release --bin bench
