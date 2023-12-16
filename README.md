# Template Rust WASM Bitburner Library

This repository serves as a template and a starting point for Bitburner players
to use Rust in Bitburner instead of JavaScript by compiling Rust into
WebAssembly and then imported into Bitburner's JavaScript.

## Prerequisites

#### Rust

Install [Rust](https://rustup.rs/) on your computer. This will allow you to compile the code.

#### WebAssembly

Open your terminal and run these commands:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

These commands will allow you to compile to WebAssembly.

## How to use

All the tasks are managed through
```bash
cargo xtask
```

To create a JS file and put it in the game first create a new child package with
```bash
cargo new --lib <SCRIPT_NAME>
```
and adjust the contents of Cargo.toml to depend on the bitburner api package
and change library type to cdylib:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
bitburner_api = {path = "../bitburner_api"}
```

The rust bindings for the API are not complete, to add new functions refer to
the definition file that can be obtained from the game by executing
```bash
cargo xtask get-definitions
```

Include your new package in the workspace at the top level Cargo.toml
```toml
[workspace]
members = [
    "bitburner_api",
    "xtask",
    "hello",
    "<SCRIPT_NAME>",
]
```

Now you can compile your code to WASM and generate JS bindings with
```bash
cargo xtask codegen
```

To help load them into the game there is a server you can launch with
```bash
cargo xtask serve
```
Connect the game to it from Options -> Remote API. The server will monitor js
output directory and upload fresh files to the game automatically whenever
codegen succeeds.
