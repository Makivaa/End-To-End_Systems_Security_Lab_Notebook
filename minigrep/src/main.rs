/*
Recreate (a very limited version) of grep using Rust
The purpose of this excercise is to get familiar with Rust's error handling and to practice writing a command line program.

Usage:
$ cargo run -- searchstring example-filename.txt

*/

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}