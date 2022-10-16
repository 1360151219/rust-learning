// use crate::garden::vegetables::Asparagus;
pub mod garden;
fn main() {
    // let a = Asparagus {};
    let a = crate::garden::vegetables::Asparagus {};
    dbg!(a);
}
