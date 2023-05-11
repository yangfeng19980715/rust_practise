use chapter11_2;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, chapter11_2::add_two(2));
}
