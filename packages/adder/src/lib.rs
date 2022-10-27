pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn ret_bool() {
        assert!(1 < 7);
    }
    #[test]
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    #[should_panic]
    fn get_panic() {
        panic!("Big Big error!")
    }
}
