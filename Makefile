all:
	wasm-pack build --dev --target web --out-name wasm --out-dir ../static/ frontend
