use std::{env, error::Error, fs};

pub struct Config<'a> {
    query: &'a str,
    file_name: &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config<'_>, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query: &args[1],
            file_name: &args[2],
            ignore_case,
        })
    }
}

pub fn grep_file(cfg: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(cfg.file_name)?;
    if cfg.ignore_case {
        for line in grep_content_insensitive(cfg.query, &content) {
            println!("{line}");
        }
    } else {
        for line in grep_content_sensitive(cfg.query, &content) {
            println!("{line}");
        }
    }
    Ok(())
}

fn grep_content_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn grep_content_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        let line_lower = line.to_lowercase();
        if line_lower.contains(&query) {
            result.push(line);
        }
    }
    result
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
