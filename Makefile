.PHONY: release
release:
	cargo build --release && ./target/release/interpreters

