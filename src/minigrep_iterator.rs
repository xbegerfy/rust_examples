use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_name: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let Some(query) = args.next() else {
            return Err("failed to parse query arguments");
        };
        let Some(file_name) = args.next() else {
            return Err("failed to parse file_name arguments");
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: query,
            file_name: file_name,
            ignore_case,
        })
    }
}

pub fn grep_file(cfg: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&cfg.file_name)?;
    if cfg.ignore_case {
        for line in grep_content_insensitive(&cfg.query, &content) {
            println!("{line}");
        }
    } else {
        for line in grep_content_sensitive(&cfg.query, &content) {
            println!("{line}");
        }
    }
    Ok(())
}

fn grep_content_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

fn grep_content_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grep_content_case_sensitive() {
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct asdfla.
";
        let query = "duct";
        let result = grep_content_sensitive(query, content);
        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn grep_content_case_insensitive() {
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct asdfla.";
        let query = "duct";
        let result = grep_content_insensitive(query, content);
        assert_eq!(vec!["safe, fast, productive.", "Duct asdfla."], result);
    }
}
