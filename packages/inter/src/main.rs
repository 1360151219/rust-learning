use std::thread;
use std::time::Duration;
mod iter;
use crate::iter as my_iter;
struct Cacher<T> {
    caculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(caculation: T) -> Cacher<T> {
        Cacher {
            caculation,
            value: None,
        }
    }
    fn value(&mut self, args: u32) -> u32 {
        if let Some(v) = self.value {
            return v;
        } else {
            let v = (self.caculation)(args);
            self.value = Some(v);
            v
        }
    }
}
// 复杂计算
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
// 业务逻辑判断
// TODO：减少calculation函数的多次调用
fn generate_workout(intensity: u32, random_number: u32) {
    let closures = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let mut c = Cacher::new(closures);
    if intensity < 25 {
        println!("Today, do {} pushups!", c.value(intensity));
        println!("Next, do {} situps!", c.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", c.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);

    my_iter::traverse_vec(vec![1, 2, 3]);
}
