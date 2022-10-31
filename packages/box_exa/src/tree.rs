use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}
#[test]
fn tree() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let root = Rc::new(Node {
        value: 300,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });
    // Weak = Rc::downgrade->Weak
    *(leaf.parent.borrow_mut()) = Rc::downgrade(&root);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
