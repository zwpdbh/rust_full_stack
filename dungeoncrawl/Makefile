build: 
	cargo build --target wasm32-unknown-unknown && wasm-bindgen --out-dir ../frontend/assets/dungeoncrawl/ --target web ../target/wasm32-unknown-unknown/release/dungeoncrawl.wasm