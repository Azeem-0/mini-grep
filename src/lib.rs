use std::{error::Error, fs};

pub struct Config {
    pub pattern: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            let pattern = args[1].clone();
            let file_path = args[2].clone();

            Ok(Config { pattern, file_path })
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents =
        fs::read_to_string(config.file_path).expect("error while opening and reading the file");

    println!("{:?}", search(&config.pattern, &file_contents));

    Ok(())
}

// searching the pattern from the file.

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

// adding test module.

#[cfg(test)]
mod tests {

    use super::search;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents)); // fome funton())
    }
}
