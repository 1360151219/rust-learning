mod largest;
mod summary;
use crate::summary::Summary;
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    //这里泛型参数 X1 和 Y1 声明于 impl 之后，因为他们与结构体定义相对应。而泛型参数 X2 和 Y2 声明于 fn mixup 之后，因为他们只是相对于方法本身的。
    fn mixup<U>(&self, other: Point<U>) -> Point<U> {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}
// 只有Point<f64>才会具有这个方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 1.2, y: 5.2 };
    let p3 = Point { x: "Hello", y: "c" };
    dbg!(p1.x()); // 1
    dbg!(p2.distance_from_origin());
    dbg!(p1.mixup(p3));

    let t1 = Tweet {
        reply: true,
        retweet: true,
        username: String::from("Tom"),
        content: String::from("hello world"),
    };
    t1.sum_default();
    let a1 = [12, 13, 14, 15, 16, 10];
    let a2 = vec!['a', 'b', 'c'];
    println!(
        "{},{}",
        largest::get_largest(&a1),
        largest::get_largest(&a2)
    );
    println!(
        "{},{}",
        largest::get_largest_retref(&a1),
        largest::get_largest_retref(&a2)
    );
}
