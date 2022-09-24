doc:
	cargo doc --open --all-features

test:
	cargo test --release --all-features

build:
	cargo build --release

build-wasm:
	rustc 

run-server:
	cargo ou