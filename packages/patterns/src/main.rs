use patterns::{four, gibberish, multi_add, mvec, repeat_two};

struct Point {
    x: i32,
    y: i32,
}
fn main() {
    // let mut stack = vec![1, 2, 3];
    let mut s2 = mvec!(1; 2; 3);
    while let Some(top) = s2.pop() {
        println!("pop:{}", top);
    }
    println!("{}", four! {});
    println!("{}", gibberish!(4 fn ['spang "whammo"] @_@));
    println!("{:?}", repeat_two!( 1 2 3 4 5 6, 7 8 9 10 11 12 ));
}

#[test]
fn destruction_struct() {
    let p = Point { x: 1, y: 2 };
    let Point { x, y } = p;
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}

#[test]
fn at_binding() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: my_id @ 3..=7 } => assert_eq!(my_id, 5),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

#[test]
fn macro_vev() {
    let v = mvec!(1; 2; 3);
    assert_eq!(vec![1, 2, 3], v);
}

#[test]
fn macro_multi_add() {
    assert_eq!(15, multi_add!(1, 2, 3, 4, 5));
}

#[test]
fn macro_rep_two() {
    assert_eq!(
        vec![1, 7, 2, 8, 3, 9, 4, 10, 5, 11, 6, 12],
        repeat_two!( 1 2 3 4 5 6, 7 8 9 10 11 12)
    )
}
