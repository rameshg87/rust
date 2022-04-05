fn main() {
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, x, y) => println!("First is zero, x is {}, y is {}", x, y),
        (1, ..) => println!("First is one, rest does not matter"),
        _ => println!("It doesn't matter what they are"),
    }
}
