use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::fmt;
use std::cmp;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String], case_sensitive: bool) -> Result<Config, &str> {
        if args.len() < 3 {
            return Result::Err("Not enough parameters");
        }
        Result::Ok(Config { query: args[1].clone(), filename: args[2].clone(), case_sensitive })
    }
}

#[derive(Debug)]
pub struct Line<'a> {
    pub number: usize,
    pub content: &'a str
}

impl<'a> Line<'a> {
    fn new(number: usize, content: &'a str) -> Line {
        Line {
            number,
            content
        }
    }
}

impl<'a> fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line #{}: {}", self.number + 1, self.content)
    }
}

impl<'a> cmp::PartialEq for Line<'a> {
    fn eq(&self, other: &Line<'a>) -> bool {
        self.number == other.number && self.content == other.content
    }
}

pub fn run(config : Config) -> Result<(), Box<Error>>{
    let mut content = String::new();
    let mut file = File::open(&config.filename)?;
    
    file.read_to_string(&mut content)?;

    for line in search(&config.query, &content, config.case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str, case_sensitive : bool) -> Vec<Line<'a>> {
    let mut results = Vec::new();

    for (line, line_number) in content.lines().zip(0..content.lines().count()) {
        if case_sensitive {
            if line.contains(query) {
                results.push(Line::new(line_number, &line));
            }
        } else {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                results.push(Line::new(line_number, &line));
            }
        }
        
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Duck";
        let content = "\
        Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(
            vec![ Line::new(3, content.lines().nth(3).unwrap()) ],
            search(query, content, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![ 
                    Line::new(0, content.lines().nth(0).unwrap()), 
                    Line::new(3, content.lines().nth(3).unwrap())
                ],
            search(query, content, false)
        );
    }
}