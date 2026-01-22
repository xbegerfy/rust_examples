use std::{fmt::Display, ops::Deref};

#[derive(Debug)]
struct MyBox<T>
where
    T: Display,
{
    data: T,
}

impl<T: Display> MyBox<T> {
    fn new(data: T) -> MyBox<T> {
        MyBox { data }
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn hello_deref_coercions(s: &str) {
    println!("s: {s}");
}

fn main() {
    let x = "111";
    let y = MyBox::new(x);
    println!("x: {x}, y: {:?}", y);
    println!("x == y: {}", x == *y);
    hello_deref_coercions(&y);
}
