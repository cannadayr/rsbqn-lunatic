cargo build --release --target=wasm32-wasi
lunatic target/wasm32-wasi/release/rsbqn-lunatic.wasm
telnet 127.0.0.1 10080
