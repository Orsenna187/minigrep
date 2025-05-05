use std::{env, process};
use minigrep::Config; // lib.rs doesn't need to be declared as a module

fn main() {
    // std::env::args returns an iterators of the command line arguments passed to minigrep
    // first value will always be the name of our binary
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1); // non-zero exit status is the convention
        // to signal that the process that called our program exited with an error state
    });

    // we use if let bc the run function doesn't return a value
    // that we want to .unwrap()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application eror: {e}");
        process::exit(1);
    }
}