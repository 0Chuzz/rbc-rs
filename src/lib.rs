






pub mod aig;
pub mod symbolic;


#[cfg(test)]
mod tests {
    use std::collections::HashMap;


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
}
