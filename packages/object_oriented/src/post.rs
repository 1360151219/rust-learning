pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}
impl Post {
    pub fn new() -> Self {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }
    pub fn add_text(&mut self, value: &str) {
        if self.state.as_ref().unwrap().change_content() {
            self.content.push_str(value);
        }
    }
    pub fn content(&self) -> &str {
        // 只有发布后才能看到内容
        self.state.as_ref().unwrap().content(self)

        // &self.content[..]
    }
    pub fn reject(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.reject())
        }
    }
    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
    }
}
trait State {
    fn change_content(&self) -> bool;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn change_content(&self) -> bool {
        true
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_time: 0 })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    approve_time: i32, // approve 2次才会Published
}
impl State for PendingReview {
    //注意不同于使用 self、 &self 或者 &mut self 作为方法的第一个参数，这里使用了 self: Box<Self>。
    //这个语法意味着该方法只可在持有这个类型的 Box 上被调用。这个语法获取了 Box<Self> 的所有权使老状态无效化，
    //以便 Post 的状态值可转换为一个新状态。
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approve_time == 0 {
            Box::new(PendingReview { approve_time: 1 })
        } else {
            Box::new(Published {})
        }
    }
    fn change_content(&self) -> bool {
        false
    }
}

struct Published {}
impl State for Published {
    //注意不同于使用 self、 &self 或者 &mut self 作为方法的第一个参数，这里使用了 self: Box<Self>。
    //这个语法意味着该方法只可在持有这个类型的 Box 上被调用。这个语法获取了 Box<Self> 的所有权使老状态无效化，
    //以便 Post 的状态值可转换为一个新状态。
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn change_content(&self) -> bool {
        false
    }
}
#[test]
fn test_post() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());
    post.add_text(",hello");
    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

#[test]
fn test_reject() {
    let mut post = Post::new();
    post.request_review();
    post.reject();
    post.add_text("I ate a salad for lunch tomorrow");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch tomorrow", post.content());
}
