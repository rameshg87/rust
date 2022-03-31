#[derive(Debug)]
struct Number {
    value: i32,
}

impl std::convert::From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    let int = 30;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
