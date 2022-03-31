#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn square(&self, len: f32) -> Rectangle {
        let top_left = Point {
            x: self.x,
            y: self.y,
        };
        let bottom_right = Point {
            x: self.x + len,
            y: self.y - len,
        };
        Rectangle {
            top_left: top_left,
            bottom_right: bottom_right,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        (self.bottom_right.x - self.top_left.x) * (self.top_left.y - self.bottom_right.y)
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 11.2, y: 0.1 };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: right_edge,
    } = point;
    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: right_edge,
        },
        bottom_right: bottom_right,
    };
    println!("area of rectangle {:?} is {}", rectangle, rectangle.area());
    let _unit = Unit;

    println!("square from top_left is {:?}", point.square(5f32));

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
