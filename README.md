```
cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/surreal-to-wasm-test.wasm --out-dir pkg --typescript --target web
```
or 
```
wasm-pack build --dev --target web
```
```
 docker run --rm --pull always -p 8000:8000 surrealdb/surrealdb:latest start --auth --user root --pass root
``` 

https://fourteenscrews.com/essays/look-ma-no-wasm-pack/
https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html
https://surrealdb.com/docs/surrealdb/embedding/rust

