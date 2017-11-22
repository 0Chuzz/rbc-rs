extern crate rbc;
use rbc::rbc::aig;
use std::collections::hash_map;

#[test]
fn test_hello() {
    let t = aig::truen();
    let empt = hash_map::HashMap::default();
    assert_eq!(t.eval(&empt), true);
}