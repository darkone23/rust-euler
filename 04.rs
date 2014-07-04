// Problem 4: Largest palindrome product
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

use std::iter::order::equals;

fn is_palindrome(num: uint) -> bool {
    let bytes = num.to_str().into_bytes();
    equals(bytes.iter(), bytes.iter().rev())
}

fn main() {
    let mut largest = 0;
    for x in range(100u, 1000) {
        for y in range(x, 1000) {
            let num = x * y;
            if num > largest && is_palindrome(num) {
                largest = num
            }
        }
    }
    println!("{} is the largest palindromic product of three digit numbers", largest);
}
