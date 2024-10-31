# Falling Rust

A falling-sand toy written using Rust, Bevy and egui.

## How to run

A release version for Windows is available at [GitHub releases](https://github.com/grunnt/falling-rust/releases).

## How to run from the code

You will need to have [Rust](https://www.rust-lang.org) installed to compile this. 

The simulation is quite CPU intensive, so you may want to run this in release mode:

```
cargo run --release
```

## How to build for the web

Falling-rust can be built as a WASM binary as well, which allows it to be run inside a webpage.

You will need to have the `wasm32-unknown-unknown` target installed. This is easily done using rustup:
```
rustup target add wasm32-unknown-unknown
```

Then falling-rust needs to be compiled for wasm, using the profile that optimizes for binary size:
```
cargo build --profile web --target wasm32-unknown-unknown
```

And finally you can generate bindings for javascript (and an index.html page) using `wasm-bindgen`:
```
wasm-bindgen --out-dir ./wasm --target web ./target/wasm32-unknown-unknown/web/falling-rust.wasm
```
