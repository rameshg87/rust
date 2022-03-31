type NanoSecond = u64;
type Inch = u64;

fn main() {
    let nanoseconds: NanoSecond = 5;
    let inches: Inch = 2;

    println!(
        "{} nanoseconds + {} inches = {} unit",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
