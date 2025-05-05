use std::{fs, env}; // environment of the process 
use std::error::Error;

// use structs to group related variables together
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// separate concerns, main should do as little as possible
impl Config {
    // calling it build instead of new bc in general, we expect new functions to never fail
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        let ignore_case = if args.len() == 4 {
            args[3] == "c" // lowercase c for ignore case
        } else {
            env::var("IGNORE_CASE").is_ok()
        };
        
        Ok(Config { query, file_path, ignore_case })
    }
}

// Box<dyn Error> is a trait object
// it means that the function will return a type that implements the Error trait.
// but we don't have to specify which particular error the return value will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    results
        .iter()
        .for_each(|&line| println!("{line}"));


    Ok(()) // looks strange, but convention for showing that we're using
    // function for its side effects only and do not expect it to return a value
}

// lifetime necessary to say that the returned slices reference contents and not query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|&line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))

    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))

    }
}