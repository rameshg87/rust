use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    let value: i32 = 8;
    let result: Result<EvenNumber, ()> = value.try_into();
    println!(
        "Trying to convert {:?} to even number, result = {:?}",
        value, result
    );

    let value: i32 = 7;
    let result: Result<EvenNumber, ()> = value.try_into();
    println!(
        "Trying to convert {:?} to even number, result = {:?}",
        value, result
    );
}
