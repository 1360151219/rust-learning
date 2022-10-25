pub trait Summary {
    fn summarize(&self) -> String;
    fn sum_default(&self) {
        println!("default: {}", &self.summarize())
    }
}

// pub fn notify(summary: &impl Summary) {
//     println!("break news :{}", summary.summarize())
// }
pub fn notify<T: Summary + Copy>(summary: &T) {
    println!("break news :{}", summary.summarize())
}
pub fn some_func<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Clone,
    U: Summary + Copy,
{
    1
}
