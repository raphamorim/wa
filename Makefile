doc:
	cargo doc --open --all-features

lint:
	cargo fmt -- --check --color always
	cargo clippy --all-targets -- -D warnings

test:
	cargo test --release --all-features

build:
	cargo build --release

example:
	cargo run --example strings --features="string"
