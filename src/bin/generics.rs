fn find_lagest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut lagest = &list[0];
    for v in list {
        if lagest < v {
            lagest = v;
        }
    }
    lagest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let values = vec![1, 34, 4, 52, 354];
    let lagest = find_lagest(&values);
    println!("lagest: {lagest}");
    let values = vec![3.2, 4.5, 2.1, 6.3];
    let lagest = find_lagest(&values);
    println!("lagest: {lagest}");

    let point = Point { x: 1, y: 2 };
    println!("point: {point:?}")
}
