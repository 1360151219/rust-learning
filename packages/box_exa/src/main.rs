use crate::BoxList::{Cons as BCons, Nil};
use crate::RcList::{Cons as RCons, Nil as RNil};
use std::{ops::Deref, rc::Rc};
mod ref_cell;
mod tree;
#[derive(Debug)]
enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil,
}
#[derive(Debug)]

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

pub struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    fn deref(&self) -> &Self::Target {
        &self.0
    }

    type Target = T;
}
struct CustomDrop {
    data: String,
}
impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("Dropping with data `{}`!", self.data);
    }
}
fn foo(s: &str) {}

fn main() {
    let b = Box::new(5);
    println!("{}", b);
    let l1 = BCons(1, Box::new(BCons(2, Box::new(BCons(3, Box::new(Nil))))));
    let rl1 = Rc::new(RCons(
        1,
        Rc::new(RCons(2, Rc::new(RCons(3, Rc::new(RNil))))),
    ));
    // let rl2 = RCons(10, Rc::clone(&rl1));
    // let rl3 = RCons(20, Rc::clone(&rl1));
    dbg!(Rc::strong_count(&rl1));
    // dbg!(l);
    let a = "slice";
    let b = String::from("slice");
    let c = &Box::new(b);
    foo(c);
    let d1 = CustomDrop {
        data: String::from("hello"),
    };
    let d2 = CustomDrop {
        data: String::from("world"),
    };
    drop(d1);
    println!("{}", 7 as f64 / 8 as f64)
}
