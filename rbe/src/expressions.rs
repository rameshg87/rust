fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x * x * x;
        x_squared + x_cube + x
    };

    let z = {
        2 * x
    };

    println!("x {} y {} z {:?}", x, y, z);
}
