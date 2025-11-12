run:
	cargo run -- -f expression.txt

test:
	cargo test

help:
	cargo run -- --help

credits:
	cargo run -- --credits

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

precommit:
	cargo fmt --all
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

clean:
	cargo clean

build:
	cargo build --release