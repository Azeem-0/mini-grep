use std::{env, process};

use minigrep::{run, Config};

fn main() {
    // reading the command line inputs.

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(error) = run(config) {
        println!("Application error : {error}");
        process::exit(1);
    }
}
