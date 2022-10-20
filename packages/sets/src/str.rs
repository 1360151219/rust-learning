pub mod str {
    pub fn add1() {
        let s1 = String::from("hello ");
        let s2 = String::from("world");
        println!("{}", s1 + &s2);
    }
    pub fn add2() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
    }
    pub fn inter() {
        let s1 = String::from("tic");
        for c in s1.chars() {
            println!("{c}");
        }
    }
}
