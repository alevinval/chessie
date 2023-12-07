.PHONY: lint
lint:
	cargo +nightly fmt
	cargo clippy --tests -- -Dclippy::all

.PHONY: test
test:
	cargo test
