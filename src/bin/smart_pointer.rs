struct SmartPointer {}

impl SmartPointer {
    fn new() -> SmartPointer {
        SmartPointer {}
    }
}

impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("smart pointer drop")
    }
}

fn main() {
    let s1 = SmartPointer {};
    let s2 = SmartPointer {};

    drop(s1);   
}
