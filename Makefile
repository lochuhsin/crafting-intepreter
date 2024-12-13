.PHONY: release
release:
	cargo build --release && ./target/release/interpreters

.PHONY: coverage
coverage:
	cargo tarpaulin --out html

# This is optional
.PHONY: install-all
install-all:
	make install-coverage

# Additional cargo packages #
.PHONY: install-coverage
install-coverage:
	cargo install cargo-tarpaulin