use std::{error::Error, fs, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        };
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config { query, filename, case_sensitive });
    }
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line: &&str| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let normalized_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| {
            let normalized_line = line.to_lowercase();
            normalized_line.contains(&normalized_query)
        })
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
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
    Pick three.\n\
    Trust me.";

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let expected = vec!["safe, fast, productive."];

        assert_eq!(expected, search_case_sensitive(query, CONTENTS));
    }

    #[test]
    fn case_insensitive() {
        let query = "ruST";
        let expected = vec!["Rust:", "Trust me."];

        assert_eq!(expected, search_case_insensitive(query, CONTENTS));
    }
}
