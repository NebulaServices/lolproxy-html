# lolproxy html rewriter

lol
`cargo build --target wasm32-unknown-unknown`
`wasm-bindgen --target web --out-dir pkg ./target/wasm32-unknown-unknown/debug/lolproxy.wasm`

for testing
`miniserve --index static/lolhtml.html`
