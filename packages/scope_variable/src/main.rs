fn main() {
    let s1 = "hello";
    let mut s2 = String::from("hello");
    s2.push_str(",world");

    let s3 = bar(s2);
    // s2 发生了移动，因此函数调用之后的语句中无法使用s2了。所有者消失了～～
    // foo(s2);

    // 引用 只使用其地址
    refer(&s3);
    // println!("s1:{}, s2:{}, s3:{}", s1, s2, s3);
    println!("s1:{}, s2, s3:{}", s1, s3);
}

fn foo(a: String) {
    println!("{a}")
}

fn bar(b: String) -> String {
    b
}
// 元组
fn car(c: String) -> (usize, String) {
    (c.len(), c)
}

fn refer(d: &String) -> usize {
    d.len()
}
