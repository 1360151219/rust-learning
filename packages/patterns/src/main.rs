use patterns::mvec;

struct Point {
    x: i32,
    y: i32,
}
fn main() {
    // let mut stack = vec![1, 2, 3];
    let mut s2 = mvec![1; 2; 3];
    while let Some(top) = s2.pop() {
        println!("pop:{}", top);
    }

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
