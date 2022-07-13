# single_player_ffi

FFI-ready wrappers around `single_player`. This is native (dynamic library) and WASM ready.

To build the project for WASM, run `cargo build -p single_player_ffi --target wasm32-unknown-unknown`. The resulting `.wasm` binary will be in the parent directory's `target` folder (`../target/wasm32-unknown-unknown/debug/single_player_ffi.wasm`, relative to this README file).
