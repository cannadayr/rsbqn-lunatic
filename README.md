Run
-----
    cargo build --target=wasm32-wasi
    lunatic target/wasm32-wasi/debug/rsbqn-lunatic.wasm
    telnet 127.0.0.1 10080
