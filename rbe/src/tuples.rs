use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn transpose(m: &Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("long_tuple first value: {}", long_tuple.0);
    println!("long_tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (-1i8, -2i16, -3i32));
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    println!("long_tuple: {:?}", long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix is\n{}", matrix);
    println!("transpose is \n{}", transpose(&matrix));
}
