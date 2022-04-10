#[derive(Debug)]
struct Pair {
    x: Box<i32>,
    y: Box<i32>,
}

impl Pair {
    fn consume(self: Self) {
        println!(
            "Pair with values {} and {} has been consumed.",
            self.x, self.y
        );
    }
}

fn main() {
    let pair = Pair {
        x: Box::new(1),
        y: Box::new(2),
    };
    println!("{:?}", pair);
    pair.consume();
}
