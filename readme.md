# trying sqlite instead of GBQ

# needed before building
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi

# build:
cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-wasi

# serve build:
trunk serve
