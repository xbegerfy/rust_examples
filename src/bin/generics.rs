fn find_lagest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut lagest = &list[0];
    for v in list {
        if lagest < v {
            lagest = v;
        }
    }
    lagest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lagest() {
        let r = vec!["hello", "worlds"];
        let reuslt = find_lagest(&r);
        assert_eq!(*reuslt, "worlds");
    }

    #[test]
    fn test_find_lagest_err() -> Result<(), String> {
        let r = vec!["hello", "worlds"];
        let reuslt = find_lagest(&r);
        if *reuslt == "worlds" {
            return Ok(());
        }
        return Err(String::from("failed"));
    }
}

#[derive(Debug)]
struct Point<T> {
    pub x: T,
    pub y: T,
}

fn main() {
    let values = vec![1, 34, 4, 52, 354];
    let lagest = find_lagest(&values);
    println!("lagest: {lagest}");
    let values = vec![3.2, 4.5, 2.1, 6.3];
    let lagest = find_lagest(&values);
    println!("lagest: {lagest}");

    let point = Point { x: 1, y: 2 };
    println!("point: {}, {}", point.x, point.y)
}
