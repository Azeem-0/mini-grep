use std::{
    env::{self},
    error::Error,
    fs,
};

pub struct Config {
    pub pattern: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let pattern = match args.next() {
            Some(pat) => pat,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(pat) => pat,
            None => return Err("Didn't get file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            pattern,
            file_path,
            ignore_case,
        })
    }
}

// running the main logic of the application.

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents =
        fs::read_to_string(config.file_path).expect("error while opening and reading the file");

    println!(
        "{:?}",
        if config.ignore_case {
            search_case_insensitive(&config.pattern, &file_contents)
        } else {
            search(&config.pattern, &file_contents)
        }
    );

    Ok(())
}

// searching the pattern from the file.

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

// adding test module.

#[cfg(test)]
mod tests {

    use super::{search, search_case_insensitive};

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
