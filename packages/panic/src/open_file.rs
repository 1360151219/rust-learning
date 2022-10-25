use std::fs::File;
use std::io::{Error, Read};
pub fn read_file() -> Result<String, Error> {
    let f = File::open("cargo2.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
