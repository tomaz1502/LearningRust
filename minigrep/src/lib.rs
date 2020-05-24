use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(config.file)?;
    println!("Text from specified file:\n{}", text);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! Please provide a pattern and a file.");
        }
        let file = args[1].clone();
        let query = args[2].clone();

        Ok(Config { query, file })
    }
}
