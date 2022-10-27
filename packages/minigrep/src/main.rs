use std::env;
use std::fs;
use std::path;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        printIn!("{}", err);
        process::exit(1);
    });
    let p = path::Path::new(&config.filename); // 项目根目录
    let res = fs::read_to_string(&p.canonicalize().unwrap()).unwrap();
    println!("{}", res);
}

#[derive(Debug)]
struct Config {
    filename: String,
}
impl Config {
    fn new(args: &Vec<String>) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(format!(
                "not enough arguements, needs 1 got {}",
                args.len() - 1
            ));
        }
        let s = &args[1];
        Ok(Config {
            filename: s.to_string(),
        })
    }
}
