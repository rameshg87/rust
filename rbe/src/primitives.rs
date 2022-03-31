fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let mut mutable = 12;
    println!("mutable {}", mutable);
    mutable = 4294967296i64;
    println!("mutable {}", mutable);
    print_type_of(&mutable);
    let mutable = true;
    println!("mutable {}", mutable);
    print_type_of(&mutable);
}
