extern crate rbc;

use std::collections::hash_map::HashMap;
use rbc::formula::Formula;

#[test]
fn test_hello() {
    let f = Formula::new();
    let empt = HashMap::default();
    assert_eq!(f.trueval().eval(&empt), true);
}


#[test]
fn it_works() {
    let f = Formula::new();
    let a = f.var(&String::from("a"));
    let b = f.var(&String::from("b"));
    let mut ass = HashMap::new();
    ass.insert(String::from("a"), true);
    ass.insert(String::from("b"), false);
    
    assert_eq!(f.or(&a, &b).eval(&ass), true);
}
