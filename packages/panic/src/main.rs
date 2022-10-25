use std::fs::File;
use std::io::ErrorKind;
mod open_file;
fn main() {
    // let f = File::open("cargo1.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("cargo1.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };
    // dbg!(f);
    match open_file::read_file() {
        Ok(s) => println!("{}", s),
        Err(e) => panic!("{}", e),
    }
}
