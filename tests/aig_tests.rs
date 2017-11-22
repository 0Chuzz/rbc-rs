extern crate rbc;
use rbc::aig;
use std::collections::hash_map;

#[test]
fn test_hello() {
    let t = aig::truen();
    let empt = hash_map::HashMap::default();
    assert_eq!(t.eval(&empt), true);
}


#[test]
fn it_works() {
    use aig::*;
    let a = var(String::from("a"));
    let b = var(String::from("b"));
    let mut ass = HashMap::new();
    ass.insert(String::from("a"), true);
    ass.insert(String::from("b"), false);
    
    or(&a, &b).eval(&ass);
}