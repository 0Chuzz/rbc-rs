use std::collections::HashMap;
use std::cmp::Ordering;
use std::option::Option;


//pub struct AigEdge;
#[derive(PartialEq, Eq, Hash)]
pub enum AigNode<'a> {
    TrueNode,
    VarNode(&'a str),
    AndNode(AigEdge<'a>, AigEdge<'a>),
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AigEdge<'a> {
    pub flip: bool,
    pub node: &'a AigNode<'a>,
}


impl<'a> PartialOrd for &'a AigNode<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((*self as *const  AigNode<'a>).cmp(&(*other as *const  AigNode<'a>)))
    }
}

impl<'a> Ord for &'a AigNode<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        ((*self as *const  AigNode<'a>).cmp(&(*other as *const  AigNode<'a>)))
    }
}
impl<'a> Clone for AigEdge<'a> {
    fn clone(&self) -> Self {
        AigEdge{flip: self.flip, node: self.node}
    }
}

impl<'a> AigEdge<'a> {
    pub fn eval(&self, assgm: &HashMap<String, bool>) -> bool {
        let n: &AigNode = &self.node;
        match n {
            &AigNode::TrueNode => !self.flip,
            &AigNode::VarNode(var) => assgm[var],
            &AigNode::AndNode(ref e1,ref e2) => self.flip ^ (e1.eval(assgm) && e2.eval(assgm)),
        }
    }
}
