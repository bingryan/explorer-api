.PHONY: run
run:
	cargo run

.PHONY: clear
clear:
	rm -rf target/

.PHONY: release
release:
	cargo build --release


.PHONY: install
install:
	cargo install --force --path .