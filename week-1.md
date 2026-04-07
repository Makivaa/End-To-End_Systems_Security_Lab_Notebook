# Overview
## Rustlings every week
## Projects via Rust Book
## Projects later in the quarter

## Rust
rustc main.rs // compile
./main
### Use Cargo
cargo new "example" // build new rust project 
cargo run // run rust project (auto debug) 
cargo run --release // smaller and faster (not debug)
cargo check // check if code compiles but don't build binary

Rust binaries contain the std library, leading to bigger sizes, because of generics.
Generics are basically type inferencing in Haskell(?), and Rust uses a monomorphism to generate
generics for the optimization of your code

## Rustlings
Rust uses the `print!` and `println!` macros to print text to the console.
cargo run --package exercises --bin intro1 // example run
Need let in front of variables