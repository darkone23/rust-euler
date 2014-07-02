// problem 01: multiples of 3 and 5
// find the sum of all the multiples of 3 or 5 below 1000

fn divides_by(n: int, x: int) -> bool { x % n == 0 }

fn main() {
    let mut nums: Vec<int> = vec![];        
    for n in range(0, 1000) {
        if divides_by(3, n) ||
           divides_by(5, n) {
            nums.push(n);
        }
    }
    let sum: int = nums.iter().fold(0, |a, &b| a + b);
    println!("The sum is: {}", sum);
}
