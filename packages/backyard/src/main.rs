// use crate::garden::vegetables::Asparagus;
mod garden;
fn main() {
    // let a = Asparagus {};
    let a = crate::garden::vegetables::Asparagus {};
    dbg!(a);
}
