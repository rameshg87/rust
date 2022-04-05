fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "6".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
