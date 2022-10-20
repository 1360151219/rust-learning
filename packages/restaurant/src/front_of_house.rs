// front_of_house 和 eat_in_restaurant 处于同一层级上，因此可以访问
// 模块公有但其内部属性和方法并不公有
pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}
pub mod serving {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}

    fn fix_incorrect_order() {
        take_order();
        // super 指指的是父模块. 当前模块serving的父模块->front_of_house
        super::serving::serve_order();
    }
}
