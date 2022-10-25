pub fn get_largest<T: Copy + PartialOrd>(arr: &[T]) -> T {
    let mut largest = arr[0];
    for &item in arr {
        if largest < item {
            largest = item
        }
    }
    largest
}

pub fn get_largest_retref<T: PartialOrd>(arr: &[T]) -> &T {
    let mut largest = &arr[0];
    let mut index = 0;
    for (i, item) in arr.iter().enumerate() {
        if largest < item {
            largest = item;
            index = i;
        }
    }
    &arr[index]
}
