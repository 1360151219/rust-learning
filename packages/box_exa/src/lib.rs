use std::ops::Deref;

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    // 自定义解引用行为
    fn deref(&self) -> &Self::Target {
        &self.0
    }

    type Target = T;
}


#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn is_ref_equal() {
        let a = 5;
        let b = &a;
        assert_eq!(5, a);
        assert_eq!(5, *b);
    }
    #[test]
    fn is_box_equal() {
        let a = 5;
        let b = Box::new(a);
        assert_eq!(5, a);
        assert_eq!(5, *b);
    }
    #[test]
    fn mybox_equal() {
        let a = 5;
        let b = MyBox::new(a);
        assert_eq!(5, *b);
    }
}
