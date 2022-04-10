enum WebEvent {
    // An enum which is unit like
    PageLoad,
    PageUnload,
    // or like tuple structs
    KeyPress(char),
    Paste(String),
    // or like c-structs
    Click { x: i64, y: i64 },
}

#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}

fn inspect(event: &WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

fn main() {
    for event in [
        WebEvent::KeyPress('x'),
        WebEvent::Paste("my text".to_owned()),
        WebEvent::Click { x: 20, y: 80 },
        WebEvent::PageLoad,
        WebEvent::PageUnload,
    ]
    .iter()
    {
        inspect(event);
    }

    println!("zero is {}", Number::Zero as i32);
}
