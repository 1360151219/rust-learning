// 这个库可以用于记录用户所允许的 API 调用数量限额。

use std::cell::RefCell;

pub trait Message {
    fn send(&self, msg: &str);
}
struct LimitTracker<'a, T: Message> {
    messager: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Message,
{
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messager,
            max,
            value: 0,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
struct MockMessenger {
    send_msg: RefCell<Vec<String>>,
}
impl Message for MockMessenger {
    fn send(&self, msg: &str) {
        self.send_msg.borrow_mut().push(String::from(msg));
    }
}
#[cfg(test)]
#[test]
fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger {
        send_msg: RefCell::new(vec![]),
    };
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(80);

    assert_eq!(
        mock_messenger.send_msg.borrow_mut()[0],
        "Warning: You've used up over 75% of your quota!"
    );
}
