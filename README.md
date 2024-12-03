# Advent of Code - 2024

## Rust Environment setup

Using ubuntu as the OS and nix as the package manager.

Install the following packages with home-manager: `rustup`, `lldb`

Install rust dev toolchain with `rustup default stable`

Install the following VSCode Extensions: `rust-lang.rust-analyzer`, `Swellaby.vscode-rust-test-adapter` and `vadimcn.vscode-lldb`



## daily setup

`cargo new day1 --lib`

- move tests to new file
- make a main.rs with `use dayXX::*` at the top to import library code

## Rust learning

Variables are owned by functions and are immutable by default. Usually you want to borrow a variable, which is kind of like passing by reference immutably.

Vectors are arrays:
- `v.len()` is the length of an array in `usize` (based on how many bits in processor)
- array chaining can be started with an `iter()` call
- iterators can be evaluated with `.collect()`
- map, filter, and split all work as expected
    - lambdas follow the following syntax
      ```rs
      |parameter| implied_return_statement
      // or
      |parameter| {
        //body calculations
        implied_return_statement
      }
      ```
    - lining up lambdas and properly borrowing vectors is a little tricky to keep track of