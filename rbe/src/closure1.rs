fn main() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let annotated_closure = |i: i32| i + 1;
    let inferred_closure = |i| i + 1;

    println!("function {}", function(1));
    println!("annotated_closure {}", annotated_closure(1));
    println!("inferred_closure {}", inferred_closure(1));
}
