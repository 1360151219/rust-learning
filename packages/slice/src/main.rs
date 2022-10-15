fn main() {
    let s = String::from("hello world code");
    let word = first_word(&s); // 5 与 s 不相关，因此当s内存释放后，这个5失效了
                               // s.clear();
    let first = first_word_slice(&s);
    let second = second_word(&s);
    let slice1 = &s[0..2]; // he
    let slice2 = &s[..3];
    println!("{}, {}", first, second)
}

fn first_word(s: &String) -> usize {
    // as_bytes 将字符串的各个char 转化成 ASCII码数组
    let bytes = s.as_bytes();
    // println!("{},{:#?}", s, bytes);
    // iter 创建了一个迭代器，而enumerate将其变成了一个元组。
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 字符串slice ： &str
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut head = 0;
    let mut tail = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && head == 0 {
            head = i;
        } else if item == b' ' {
            tail = i;
            return &s[head..tail];
        }
    }
    return &s[head..];
}
