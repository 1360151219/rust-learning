fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    let z = z();
    let c = foo();
    println!("y:{}, z: {}, char:{c}", y, z)
}

fn z() -> i32 {
    5
}

// char 一个字母
fn foo() -> char {
    return 'a';
}
