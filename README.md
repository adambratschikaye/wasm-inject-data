# wasm-inject-data
Example of injecting a data section into an existing wasm module.

This repo shows how a wasm module can be created which uses static data that is expected to be added at a later time.
There are two artifacts:

## `static-wasm`
This is a wasm module which reads a passive data section into a vector.
The passive data section is not actually present when the wasm module is built.
Instead, it is expected that the additional data section be injected later.

## `inject`
This is a native binary that can mutate `static-wasm` to inject new data as an additional passive data segment.

## Usage
The script `test.sh` shows how to build `static-wasm`, then use `inject` to add a data segment, and then run the resulting wasm with `wasmtime`.
