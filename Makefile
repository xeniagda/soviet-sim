build:
	cargo +nightly build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/rougelike.wasm site

debug:
	cargo +nightly build --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/rougelike.wasm site

run:
	open site/index.html