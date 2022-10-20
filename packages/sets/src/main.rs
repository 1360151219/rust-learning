mod hash_map;
mod str;
use crate::str::str as my_str;
// Vector
fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(4);
    v1.push(5);
    v1.push(6);
    let third = &v1[0];

    match Some(third) {
        Some(thir) => println!("{}", thir),
        None => println!("none"),
    }
    for v in &mut v1 {
        *v += 1;
    }
    println!("{:#?},{:#?}", v1, v2);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Text(String),
        Int(i32),
        Float(f64),
    }

    let v3 = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("hello")),
    ];
    // println!("{:#?}", v3.pop());
    my_str::add1();
    my_str::add2();
    my_str::inter();
    hash_map::hash1();
}
