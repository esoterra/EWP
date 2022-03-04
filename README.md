# Embeddable WASM Parsers

An experimental project demonstrating how WebAssembly can be used to create a standard portable representation for parsers.

## Contents

* `parser.wit` - The basic version 0 EWP interface
* `ewp-json` - an implementation of the `parser1.wit` interface for the JSON format
* `tree-ewp` - An application that accepts an EWP parser and uses it to render a text parse tree

## Running the example

### Building `ewp-json`
```sh
cd ewp-json
cargo build
cargo build --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/ewp_json.wasm ../
```

### Running `tree-ewp`
```sh
cd tree-ewp
cargo run -- ../ewp_json.wasm ../test.json
```