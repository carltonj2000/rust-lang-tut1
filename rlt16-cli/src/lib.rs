use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive =
            env::var("CASE_INSENSITIVE").is_err() && !(args.len() > 3 && args[3] == "-i");

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
    pub fn show(&self) {
        let c = if self.case_sensitive {
            "case sensitive"
        } else {
            "case insensitive"
        };
        println!("In file {} found '{}' ({}).", self.filename, self.query, c);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    config.show();
    for line in search(&config.query, &contents, &config.case_sensitive) {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, &true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust Me.";

        assert_eq!(vec!["Rust:", "Trust Me."], search(query, contents, &false));
    }
}

pub fn search<'a>(q: &str, c: &'a str, cs: &bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in c.lines() {
        let mtch = match cs {
            false => line.to_lowercase().contains(&q.to_lowercase()),
            _ => line.contains(&q),
        };
        if mtch {
            results.push(line)
        }
    }
    results
}
