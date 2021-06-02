use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        };
        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config { query, filename });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let filtered: Vec<&str> = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
    filtered
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
    Rust:\n\
    safe, fast, productive.\n\
    Pick three.";

    #[test]
    fn one_result() {
        let query = "duct";
        let expected = vec!["safe, fast, productive."];

        assert_eq!(expected, search(query, CONTENTS));
    }
}
