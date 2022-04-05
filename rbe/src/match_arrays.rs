fn main() {
    let array = [4, -2, 6];

    match array {
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => println!("array[0] = 1, array[1] was ignored, array[2] = {}", third),
        [-1, second, _] => println!("array[0] = -1, array[1] = {}, array[2] was ignored", second),
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {}, remaining = {:?}",
            second, tail
        ),
        [_, middle @ .., last] => println!(
            "array[0] was ignored, middle = {:?}, last = {}",
            middle, last
        ),
    }
}
