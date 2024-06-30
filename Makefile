.PHONY: wasm-build
wasm-build:
	cd src/lib/wasm && wasm-pack build --target web
