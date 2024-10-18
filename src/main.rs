use std::{env, process};

use minigrep::{run, Config};

fn main() {
    // reading the command line inputs

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("Application error : {error}");
        process::exit(1);
    }
}
