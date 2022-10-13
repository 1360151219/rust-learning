use std::io;
fn main() {
    let array = [10; 5];
    let mut input = String::new();
    // println!("please type a index");
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("fail to read the line");
    // let input: usize = input.trim().parse().expect("please type a index");
    // println!("{}", array[input]);

    // for in 遍历数组
    for v in array{
        println!("{v}")
    }

    // range 
    for v in (1..=4).rev(){
        println!("==, {v}")
    }
}