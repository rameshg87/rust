fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a {}", text)
}

fn main() {
    create_fn()();

    let vec1 = vec![1, 2, 3, 4];
    println!(
        "are there even numbers {:?}",
        vec1.iter().any(|&x| x % 2 == 0)
    );

    println!("one even number {:?}", vec1.iter().find(|&x| x % 2 == 0));
}
