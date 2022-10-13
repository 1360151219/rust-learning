fn main() {
    let mut a = if true { 1 } else { 2 };
    let b = loop {
        a += 1;
        if a == 10 {
            break a * 2;
        }
    };
    let c = fibonacci(10);
    println!("{a}, b:{b}, c:{c}");
}

fn foo() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// 斐波那契数列
fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    };
    let mut num1 = 1;
    let mut num2 = 1;
    for _ in 1..n {
        let t = num1 + num2;
        num1 = num2;
        num2 = t;
    }
    return num1;
}
