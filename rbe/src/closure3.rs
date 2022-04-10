use std::mem;

fn apply<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn main() {
    let mut message = "hello".to_owned();

    let handle_message = || {
        println!("message `{}`", message);

        message.push_str(" world !!");
        println!("message `{}`", message);

        // mem::drop(message);
    };

    apply(handle_message);
}
