use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
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
    if *cs {
        c.lines().filter(|line| line.contains(q)).collect()
    } else {
        c.lines()
            .filter(|line| {
                let llc = line.to_lowercase();
                let qlc = q.to_lowercase();
                llc.contains(&qlc)
            })
            .collect()
    }
}
