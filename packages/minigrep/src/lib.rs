use std::env;
use std::error::Error;
use std::fs;
use std::path;
use std::result;

#[derive(Debug)]
pub struct Config {
    filename: String,
    query: String,
    is_case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err(format!(
        //         "not enough arguements, needs 2 got {}",
        //         args.len() - 1
        //     ));
        // }
        // let q = &args[1];
        // let s = &args[2];
        args.next();
        let q = match args.next() {
            Some(arg) => arg,
            None => return Err("not get the query"),
        };
        let s = match args.next() {
            Some(arg) => arg,
            None => return Err("not get the fileName"),
        };
        let is_case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config {
            query: q,
            filename: s,
            is_case_sensitive,
        })
    }
}
#[derive(Debug, PartialEq)] // 结构体比较需要加上这个
pub struct Line<'a> {
    line: &'a str,
    index: usize,
}
impl<'a> Line<'a> {
    pub fn new(line: &'a str, index: usize) -> Line<'a> {
        Line { line, index }
    }
}
// 环境变量 mac：CASE_SENSITIVE=1 cargo run BoDy poem.txt
pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let p = path::Path::new(&conf.filename); // 项目根目录
    let real_path = p.canonicalize()?;
    let res = fs::read_to_string(real_path)?;
    if conf.is_case_sensitive {
        for line in search_case_insensitive(&conf.query, &res) {
            println!("{}: {}", line.index, line.line);
        }
    } else {
        for line in search(&conf.query, &res) {
            println!("{}: {}", line.index, line.line);
        }
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<Line<'a>> {
    // let mut res: Vec<Line> = Vec::new();
    // for (idx, line) in contents.lines().enumerate() {
    //     if line.contains(query) {
    //         res.push(Line { line, index: idx })
    //     }
    // }
    // res
    contents
        .lines()
        // .filter(|line| line.contains(query))
        .enumerate()
        .map(|(i, l)| {
            Line { line: l, index: i }
        })
        .filter(|line| line.line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<Line<'a>> {
    let mut res: Vec<Line> = Vec::new();
    for (idx, line) in contents.lines().enumerate() {
        // to_lowercase() 方法是创建一个新数据，返回一个String
        if line.to_lowercase().contains(&query.to_lowercase()) {
            res.push(Line { line, index: idx })
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let v: Vec<Line> = vec![Line::new("safe, fast, productive.", 1)];
        assert_eq!(v, search(query, contents));
    }
    #[test]
    fn test_search_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let v: Vec<Line> = vec![Line::new("Rust:", 0), Line::new("Trust me.", 3)];
        assert_eq!(v, search_case_insensitive(query, contents));
    }
}
