#[derive(PartialEq, Eq, Debug)]
struct Shoe {
    style: String,
    size: u64,
}
fn fit_shoe(shoes: Vec<Shoe>, fit_size: u64) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == fit_size).collect()
}
#[cfg(test)]
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
#[test]
fn map_adaptor() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| 2 * x).collect();
    assert_eq!(vec![2, 4, 6], v2);
}
#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = fit_shoe(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    // let b: Vec<_> = Counter::new().zip(Counter::new().skip(1)).collect();
    //      [
    //     (
    //         1,
    //         2,
    //     ),
    //     (
    //         2,
    //         3,
    //     ),
    //     (
    //         3,
    //         4,
    //     ),
    //     (
    //         4,
    //         5,
    //     ),
    // ]
    assert_eq!(18, sum);
}
