use std::collections::HashMap;
pub fn hash1() {
    let mut h1 = HashMap::new();
    h1.insert(String::from("hello"), "world");

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let h2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    let s1 = String::from("z1");
    let s2 = String::from("z2");
    let mut h3 = HashMap::new();
    h3.insert(s1, 100);
    // dbg!(h1);
    // dbg!(h2);
    // dbg!(h3);
    // dbg!(h3.get(&s1));
    // for (k, v) in &h3 {
    //     println!("{}:{}", k, v);
    // }
    h3.entry(String::from("z1")).or_insert(50);
    h3.entry(String::from("z2")).or_insert(50);
    h3.entry(String::from("z1")).or_insert(50);
    dbg!(h3);

    let text = "hello world wonderful world";
    let mut h4 = HashMap::new();
    for word in text.split_whitespace() {
        // word 单个单词
        let count = h4.entry(word).or_insert(0);
        *count += 1;
    }

    dbg!(h4);
}
