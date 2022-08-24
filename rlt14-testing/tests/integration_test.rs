// to run only tests in this file use:
// cargo test --test intergration_test

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rlt14_testing::add_two(2))
}
