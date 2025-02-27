Test HTML page with WASM served via GitHub pages

### Install dependencies

```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

### Build

```
cd wasm-test
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ../pkg --target web target/wasm32-unknown-unknown/release/wasm_test.wasm
```
