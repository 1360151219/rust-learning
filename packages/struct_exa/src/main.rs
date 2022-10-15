#[derive(Debug)]
struct User {
    username: String,
    password: String,
    active: bool,
    id: u32,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, r: &Rectangle) -> bool {
        let origin_area = self.width * self.height;
        let compare_area = r.width * r.height;
        if origin_area > compare_area {
            true
        } else {
            false
        }
    }
    // 关联函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let user1 = User {
        username: String::from("Tony"),
        password: String::from("123456"),
        active: true,
        id: 99,
    };
    let user2 = User {
        id: 100,
        username: String::from("Bob"),
        ..user1
    };
    dbg!(&user2);

    // user1.password 是无法使用的，被转移了～ 而username可以。
    // println!("{}, {}", user1.username, user2.username);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // 关联函数调用
    let squ = dbg!(Rectangle::square(3));

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
