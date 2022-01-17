check:
	cargo +nightly fmt --check
	cargo clippy -- -W clippy::pedantic
	cargo test
