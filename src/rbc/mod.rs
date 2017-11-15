pub mod aig;


    extern crate ref_eq;
    use self::ref_eq::ref_eq;


    use std::collections::HashMap;
    use std::hash::*;
    use std::string::String;
    use std::rc::Rc;
    use std::vec::Vec;
    use std::any::Any;
    use std::borrow::Borrow;

    

    trait BooleanLike<B> {
        fn truth() -> B;
        fn conjunction(&B, &B) -> B;
    }
/*
    trait RbcOperation {
        fn eval_symbolic(&self, HashMap<RbcNode, >) -> RbcNode;

    }

    pub struct RbcNode<op> {
        op: &'static RbcOperation,
        children: Vec<(bool, RbcNode)>,
    }
    impl PartialEq for RbcNode {
        fn eq(&self, other: &RbcNode) -> bool {
            return (self.op as *const RbcOperation) == (other.op as *const RbcOperation) &&
                self.children == other.children;
        }
    }

    impl Eq for RbcNode {}



    impl Hash for RbcNode {
        fn hash<H: Hasher>(&self, h: &mut H) {
            h.write_usize(self.op.as_usize());
            self.children.hash(h);

        }
    }

    struct TrueNode {}
    impl RbcOperation for TrueNode {
                fn as_usize(&self) -> usize {
                    return self as *const _ as usize;
                }
                fn eval_symbolic(&self, ass: _) -> RbcNode {
                    return self;
                }

    }*/
