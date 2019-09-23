use std::collections::HashMap;
use crate::formula;
use crate::aig;

use generic_array::*;
use generic_array::sequence::GenericSequence;

pub trait BooleanLike {
    fn truth() -> Self;
    fn conjunction(a: &Self, b: &Self) -> Self;
    fn negation(a: &Self) -> Self;
}

pub trait SymbolicEval<K, B: BooleanLike + Clone> {
    fn eval_symbolic(&self, assigment: &mut HashMap<K, B>) -> B;
}

impl<N:ArrayLength<bool>> BooleanLike for GenericArray<bool,N> {
    fn truth() -> Self {
        return Self::generate(|_| true);
        }
    fn conjunction(a: &Self, b: &Self) -> Self {
        let mut ret = Self::default();
        for (i,(&ai,&bi)) in a.iter().zip(b.iter()).enumerate(){
            ret[i] = ai && bi;
        }
        return ret;
    }
    fn negation(a:&Self) -> Self {
        let mut ret = Self::default();
        for (i,&ai) in a.iter().enumerate(){
            ret[i] = !ai;
        }
        return ret;
    }
} 
/*
impl<'a> BooleanLike for aig::AigEdge<'a> {
    fn truth() -> Self{
        formula::truen()
    }
    fn conjunction(a:&Self, b:&Self) -> Self {
        aig::and(a,b)
    }
    fn negation(a:&Self) -> Self {
        aig::not(a)
    }
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
}*/

