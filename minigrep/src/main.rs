/*
The first task is to make minigrep accept its two command line arguments: 
the file path and a string to search for. That is, we want to be able to run 
our program with cargo run, two hyphens to indicate the following arguments 
are for our program rather than for cargo, a string to search for, and a 
path to a file to search in, like so:

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