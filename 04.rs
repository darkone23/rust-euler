// Problem 4: Largest palindrome product
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn split_into_halves<T: Clone>(vec: Vec<T>) -> (Vec<T>, Vec<T>) {
    let len = vec.len();
    let mid = len / 2;
    let former = vec.slice(0, mid);
    let latter = vec.slice(mid, len);
    (Vec::from_slice(former), Vec::from_slice(latter))
}

fn is_palindrome(num: uint) -> bool {
    let char_bytes = num.to_str().into_bytes();
    let (xs, ys) = split_into_halves(char_bytes);
    let mut palindrome = true;
    for (x, y) in xs.iter().zip(ys.iter().rev()) {
        if x != y {
            palindrome = false;
            break;
        }
    }
    palindrome
}

fn max(x: uint, y: uint) -> uint {
    if x > y { x } else { y }
}

fn main() {
    let mut nums: Vec<uint> = vec![];
    for x in range(100u, 1000) {
        for y in range(100u, 1000) {
            let num = x * y;
            if is_palindrome(num) { nums.push(num); }
        }
    }
    let largest = nums.iter().fold(0, |a, &b| max(a, b));
    println!("{} is the largest palindromic product of three digit numbers", largest);
}
