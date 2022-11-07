pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!(
            "[Button] width:{}, height:{}, label:{}",
            self.width, self.height, self.label
        )
    }
}

pub struct SelectBox {
    pub width: i32,
    pub height: i32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("[SelectBox] options:{:?}", self.options)
    }
}
