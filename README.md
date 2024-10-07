# symphony-of-the-stars-Rust
Código feito para outro projeto. Esse código gerar um arquivo para ser utilizado na Web, é compilado um WebAssembly (WASM para o mais íntimos).

## Packages
### Rust
###
```
wasm-bindgen = "0.2.93"
image = "0.23.14"
rand = "0.8.5"
hound = "3.4.0"
getrandom = { version = "0.2", features = ["js"] }
```
##
### Usage
#### To compile Rust language code to WebAssembly:

```
If you haven't yet: cargo install wasm-bindgen-cli

Build: cargo build --target wasm32-unknown-unknown --release

Build to WebAssembly: wasm-bindgen target/wasm32-unknown-unknown/release/wasm.wasm --out-dir ./pkg --target web

```
##
### Link do projeto principal

<a>https://github.com/rafaseto/symphony-of-the-stars</a>