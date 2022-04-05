fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    println!("numbers: {:?}", numbers);
    let even_numbers = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect::<Vec<i32>>();
    println!("even numbers: {:?}", even_numbers);
}
