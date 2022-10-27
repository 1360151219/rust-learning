mod longest;
use std::result;

use longest::longest;

struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    let novel = String::from("hello world. happy every day");
    let first = &novel.split('.').next();
    dbg!(novel.split('.').collect::<Vec<&str>>());
    dbg!(first);
}
