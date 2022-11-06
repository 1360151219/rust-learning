use std::{
    sync::mpsc::channel,
    thread::{sleep, spawn},
    time::Duration,
};
// 通过multiple productor single consumer的channel来进行通讯
pub fn transform_in_channle() {
    let (tx, rx) = channel();
    let handler = spawn(move || {
        tx.send("hello world").unwrap();
    });
    println!("{}", rx.recv().unwrap());
}

pub fn transform_vec() {
    let (tx, rx) = channel();
    spawn(move || {
        let ss = vec!["hello", "world", "rust"];
        for s in ss {
            tx.send(s).unwrap();
            sleep(Duration::from_millis(500));
        }
    });
    for r in rx {
        println!("{}", r);
    }
}

pub fn transform_vec_both() {
    let (tx, rx) = channel();
    let tx1 = tx.clone();
    spawn(move || {
        let ss = vec!["hello", "world", "rust"];
        for s in ss {
            tx.send(s).unwrap();
            sleep(Duration::from_millis(500));
        }
    });
    spawn(move || {
        let ss = vec!["1", "2", "3"];
        for s in ss {
            tx1.send(s).unwrap();
            sleep(Duration::from_millis(500));
        }
    });
    for r in rx {
        println!("{}", r);
    }
}
