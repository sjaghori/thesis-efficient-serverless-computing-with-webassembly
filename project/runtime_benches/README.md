# WebAssembly Runtime Benchmarks

This repository contains the code for the master thesis evaluation.

## Reproduction

- Install stable version of Rust
- Each benchmark has it's own `*.rs` file
    * E.g. `instantiation.rs` contains the benchmarks related to the instantiation
    * A benchmark can be initiated with the following command: e.g. `cargo bench --bench instantiation`

> The benchmarks included in this repository use `criterion.rs` library, a statistics-driven benchmarking library for Rust. 
> see more on: https://github.com/bheisler/criterion.rs
