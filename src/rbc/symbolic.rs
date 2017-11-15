use std::collections::HashMap;
use std::vec;
use super::aig;

pub trait BooleanLike {
    fn truth() -> Self;
    fn conjunction(&Self, &Self) -> Self;
    fn negation(&Self) -> Self;
}

pub trait SymbolicEval<K, B: BooleanLike + Clone> {
    fn eval_symbolic(&self, &mut HashMap<K, B>) -> B;
}


impl<B : BooleanLike + Clone > SymbolicEval<String, B> for aig::AigEdge {
    fn eval_symbolic(&self, a : &mut HashMap<String, B> ) -> B {
        let ret = self.node.eval_symbolic(a);
        if self.flip {
            B::negation(&ret)
        } else {
            ret
        }
    }
}

impl<B : BooleanLike + Clone> SymbolicEval<String, B> for aig::AigNode {
    fn eval_symbolic(&self, assgm : &mut HashMap<String, B> ) -> B {
        match self {
            &aig::AigNode::TrueNode => B::truth(),
            &aig::AigNode::VarNode(ref name)  => assgm[name].clone(),
            &aig::AigNode::AndNode(ref left, ref right) => {
                let l = left.eval_symbolic(assgm);
                let r = right.eval_symbolic(assgm);
                B::conjunction(&l, &r)
            }
        }
    }
}

