enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Self::Nil
    }

    fn prepend(self, elem: u32) -> List {
        Self::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match self {
            Self::Cons(_, ref tail) => 1 + tail.len(),
            Self::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match self {
            Self::Cons(elem, ref tail) => format!("{} {}", elem, tail.stringify()),
            Self::Nil => format!("Nil"),
        }
    }
}

fn main() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
