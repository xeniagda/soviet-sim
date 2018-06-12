build:
	cargo +nightly build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/soviet_sim.wasm site

debug:
	cargo +nightly build --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/debug/soviet_sim.wasm site

run:
	open site/index.html
