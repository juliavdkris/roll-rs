check:
	cargo fmt --check
	cargo clippy -- -W clippy::pedantic
	cargo test
