use std::mem;

fn main() {
    let color = String::from("green");

    let print = || println!("the color is {}", color);

    print();
    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();
    let _reborrow = &count;

    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    consume();
}
