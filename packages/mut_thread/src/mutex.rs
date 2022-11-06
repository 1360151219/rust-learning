use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

pub fn use_mutex() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 100;
    }
    println!("{:#?}", m);
}

pub fn use_mutex_multiThread() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let h = spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(h);
    }
    for h in handlers {
        h.join().unwrap();
    }
    println!("{}", *counter.lock().unwrap());
}
