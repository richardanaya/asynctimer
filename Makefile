build:
	@cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/asynctimer.wasm .
serve:
	python3 -m http.server 8080