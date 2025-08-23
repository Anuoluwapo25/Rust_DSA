#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_string_reverse() {
        let reverse = reverse_string("hello");
        assert_eq!(reverse, "olleh");

    }

    #[test]
    fn test_mx() {
        let max = vec![1, 2, 8];
        let max = find_max(&max);
        assert_eq!(max, Some(8));
    }
}
