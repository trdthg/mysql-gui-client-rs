echo "[INFO] Building app..."
cargo build -p client-rs --release --lib --target wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/release/client_rs.wasm --out-dir web --no-modules --no-typescript

cd web
basic-http-server --addr 127.0.0.1:8081 .