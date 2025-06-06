use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // discard program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file name"),
        };

        let ignore_case = !(env::var("GREP_IGNORE_CASE").is_err());

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.ignore_case {
        search_ignore_case(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower: String = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lower) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "foo";
        let contents = "some
more
words
but one line with foo";

        assert_eq!(vec!["but one line with foo"], search(query, contents));
    }

    #[test]
    fn no_results() {
        let query = "foo";
        let contents = "no
matches";

        let results = search(query, contents);
        assert!(results.is_empty());
    }

    #[test]
    fn multiple_results() {
        let query = "foo";
        let contents = "no foo
bar
matches foo";

        assert_eq!(vec!["no foo", "matches foo"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "FOo";
        let contents = "some
more
words
but one line with foo
and one with Foo";

        assert_eq!(
            vec!["but one line with foo", "and one with Foo"],
            search_ignore_case(query, contents)
        );
    }
}
