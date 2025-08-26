//1-- Write a function that accepts a string and returns its reverse.
//--char() act as an iterator over the string slice 
//--rev() reverses the string 
//--collect() collects the result of the reverse of string slice into the new string since we are returning a string

pub fn reverse_string(string: &str) -> String {
    string.chars().rev().collect()
}



//2-- Write a function find_max that takes a slice of numbers (e.g., &[i32]) and returns the maximum value as an Option<i32> (to handle empty slices).

//--Ord: allows comparison
//--Copy: allows copying from bit of memeory to another
//--T: generic type
//--is_empty(): checks if vector of slice is empty


pub fn find_max<T: Ord + Copy>(slice: &[T]) -> Option<T> {
    if slice.is_empty() {
        return None;
    }
    else {
        let mut max = slice[0];
        for &item in slice.iter() {
            if item > max {
                max = item;
            }
        }
        return Some(max)
    }

}

pub fn find_max_iter<T: Ord + Copy>(slice: &[T]) -> Option<T> {
    slice.iter().copied().max()
}


//3--Print numbers from 1 to n. For multiples of 3, print "Fizz"; for multiples of 5, print "Buzz"; for both, print "FizzBuzz"
//--match helps to check  if tuple conditions(i % 3, i % 5) will return true or false


fn fizzbuzz(n: u32) {
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),  
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}

//other method using map and closure which returs Vec


//4--Check if a given string is a palindrome (ignoring case and non-alphanumeric characters).

fn is_palindrome(string: &str) -> bool {
    true
}
