use std::fmt::Display;

pub fn traverse_vec<T: Display>(vec: Vec<T>) -> Vec<T> {
    // vec.into_iter() 会获取vec的所有权
    for v in vec.iter() {
        println!("{}", v);
    }
    vec
    // vec.iter().sum()
}
