all:
	cargo build
	wasm-pack build --target web --out-name wasm --out-dir ../static/ frontend
