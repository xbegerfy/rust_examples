use rust_examples::cal;

mod common;

#[test]
fn test_add() {
    common::setup();
    let r = cal::add(4, 5);
    assert_eq!(r, 9);
}
