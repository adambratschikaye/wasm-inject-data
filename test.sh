set -euxo pipefail

# Build the static wasm to be mutated
RUSTFLAGS='-C target-feature=+bulk-memory' cargo +nightly build --target wasm32-unknown-unknown --bin static-wasm

# inject the data section into the wasm module
cargo run --bin inject target/wasm32-unknown-unknown/debug/static-wasm.wasm output.wasm

# Run the modified wasm with wasmtime
wasmtime output.wasm --invoke foo