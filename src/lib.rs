use std::{error::Error, fs, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_cases: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };

        let ignore_cases = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_cases })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    let results;

    if config.ignore_cases {
        results = search_case_insensitive(&config.query, &contents)
    } else {
        results = search(&config.query, &contents)
    }

    for line in results {
        println!("{line}");
    };

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive\nPick three.\nDuck tape.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuck tape.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}
