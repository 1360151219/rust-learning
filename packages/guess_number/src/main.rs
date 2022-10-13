use rand::Rng;
use std::cmp::Ordering;
// std 标准库 类似于node自带的
use std::io;

fn main() {
    println!("Guess Number~ please input your number:");
    // let a = 5 默认是不可变的，带上 mut 即可变
    // String:: 表示是类型的静态方法，而不是实例
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            // read_line()方法接受一个参数，表示将用户输入的内容 追加 到参数上（必须是可变变量）。&mut 表示引用
            .read_line(&mut guess)
            //Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并
            //显示当做参数传递给 expect 的信息。如果 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果。
            //如果 Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。
            .expect("fail to read the line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("too large!"),
            Ordering::Less => println!("too small!"),
            Ordering::Equal => {
                println!("congraluation");
                break;
            }
        }
    }

    // println!("x = {} and y = {}", x, y);
}
