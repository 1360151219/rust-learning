use std::{
    thread::{self, sleep},
    time::Duration,
};
mod channel;
mod mutex;
fn main() {
    // let handler = thread::spawn(|| {
    //     for i in 1..6 {
    //         println!("In new thread: {}", i);
    //         sleep(Duration::from_millis(500));
    //     }
    // });
    // // 这里会阻塞主线程
    // handler.join().unwrap();
    // // 这里由于主线程结束了 因此其他线程也自动结束
    // for i in 1..3 {
    //     println!("In the main thread: {}", i);
    //     sleep(Duration::from_millis(500));
    // }
    // handler.join().unwrap();

    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // channel::transform_in_channle();
    // channel::transform_vec_both();
    // channel::transform_vec();
    // mutex::use_mutex();
    mutex::use_mutex_multiThread();
    handle2.join().unwrap();
}
