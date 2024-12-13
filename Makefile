.PHONY: release
release:
	cargo build --release && ./target/release/interpreters


.PHONY: install-all
install-all:
	make install-coverage

# Additional cargo packages #
.PHONY: install-coverage
install-coverage:
	cargo install cargo-tarpaulin