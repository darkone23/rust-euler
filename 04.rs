// Problem 4: Largest palindrome product
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn split_into_halves<'r, T>(vec: &'r [T]) -> (&'r [T], &'r [T]) {
    let len = vec.len();
    let mid = len / 2;
    let former = vec.slice(0, mid);
    let latter = vec.slice(mid, len);
    (former, latter)
}

fn is_palindrome(num: uint) -> bool {
    let char_bytes = num.to_str().into_bytes();
    let (xs, ys) = split_into_halves(char_bytes.as_slice());
    xs.iter().zip(ys.iter().rev()).all(|(x,y)| x == y)
}

fn main() {
    let mut largest = 0;
    for x in range(100u, 1000) {
        for y in range(100u, 1000) {
            let num = x * y;
            if num > largest && is_palindrome(num) {
                largest = num
            }
        }
    }
    println!("{} is the largest palindromic product of three digit numbers", largest);
}
