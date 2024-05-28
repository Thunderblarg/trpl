use std::error::Error;
use std::fs;
use std::env;

//////////////////////////////////////////////////
/// Functions go here
//////////////////////////////////////////////////

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // // Iterate through each line of the contents.
    // for line in contents.lines() {
    //     // Check whether the line contains our query string.
    //     if line.contains(&query){
    //         // If it does, add it to the list of values we’re returning.
    //         results.push(line);
    //     }
    //     // If it doesn’t, do nothing.
    // }
    // // Return the list of results that match.
    // return results;

    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    // Iterate through each line of the contents.
    for line in contents.lines() {
        // Check whether the line contains our query string.
        if line.to_lowercase().contains(&query){
            // If it does, add it to the list of values we’re returning.
            results.push(line);
        }
        // If it doesn’t, do nothing.
    }
    // Return the list of results that match.
    return results;
}

//////////////////////////////////////////////////
/// Structs and their impls go here
//////////////////////////////////////////////////

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    pub fn build(
            mut args: impl Iterator<Item = String>,
        ) -> Result<Config, &'static str> {
        args.next();

        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        // let query = args[1].clone();
        // let file_path = args[2].clone();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config {
            query,
            file_path,
            ignore_case
        });
    }
}


//////////////////////////////////////////////////
/// Tests go here
//////////////////////////////////////////////////

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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

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
}
