use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Text to be searched: {}", config.query);
    println!("Looking in file: {}", config.file);

    if let Err(e) = minigrep::run(config) {
        println!("Error during execution: {}", e);
        process::exit(1);
    }
}
