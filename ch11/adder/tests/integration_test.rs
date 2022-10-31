extern crate adder;

mod common;

#[test]
fn integration_add_two() {
    let setup1 = common::setup();
    println!("{:?}", setup1);
    assert!(
        4 == adder::add_two(2),
        "{}", setup1
    );
}
