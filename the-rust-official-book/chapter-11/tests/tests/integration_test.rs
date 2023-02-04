use tests;
mod common;
#[test]
fn it_does() {
    common::setup();
    assert_eq!(10, tests::prints_and_returns_10(1));
}
