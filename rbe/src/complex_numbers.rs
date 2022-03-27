use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} + {1}i", self.real, self.imag)
    }
}

fn main() {
    let cn1 = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", cn1);
    println!("Debug: {:?}", cn1);
}
