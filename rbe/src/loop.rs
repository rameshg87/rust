fn main() {
    let mut i = 0;

    'outer: loop {
        i = i + 1;
        if i == 5 {
            break 'outer;
        }
        println!("i {}", i);
    }
    println!("end of loop");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter {} result {}", counter, result);
}
