// Write a function that accepts a string and returns its reverse.
//--char() act as an iterator over the string slice 
//--rev() reverses the string 
//--collect() collects the result of the reverse of string slice into the new string since we are returning a string

pub fn reverse_string(string: &str) -> String {
    string.chars().rev().collect()
}
#[cfg(test)]

mod test {
    use super::*;

    #[test]

    fn test_string_reverse() {
        let reverse = reverse_string("hello");
        assert_eq!(reverse, "olleh");

    }
}
