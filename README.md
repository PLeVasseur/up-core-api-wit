# A WASI p3 WIT Demo

## Install prerequisites

```shell
cargo install cargo-component --locked
cargo install wasm-tools
cargo install wasmtime-cli
```

## Initializing a new component

```shell
cargo component new asyncdemo --lib
```

## Generating component bindings

```shell
cargo component bindings
```

## Building the generated bindings

```shell
cargo component build
```

