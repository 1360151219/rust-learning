// 集成测试
use adder;

#[test]
fn it_add_two() {
    assert_eq!(adder::add(3, 4), 7);
}
