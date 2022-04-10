fn is_odd(x: i32) -> bool {
    x % 2 != 0
}

fn main() {
    let upper = 100;
    // Sum of square of all natural numbers whose
    // 1. Square is an odd number
    // 2. Square is less than upper bound
    let sum: i32 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < upper)
        .filter(|&x| is_odd(x))
        .fold(0, |acc, x| acc + x);
    println!("sum is {}", sum);
}
