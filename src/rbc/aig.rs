use std::collections::HashMap;
use std::rc::*;


//pub struct AigEdge;
#[derive(PartialEq, Eq, Hash)]
pub enum AigNode {
    TrueNode,
    VarNode(String),
    AndNode(AigEdge, AigEdge),
}

#[derive(PartialEq, Eq, Hash)]
pub struct AigEdge {
    pub flip: bool,
    pub node: Rc<AigNode>,
}

impl Clone for AigEdge {
    fn clone(&self) -> AigEdge {
        AigEdge {
            flip: self.flip,
            node: self.node.clone(),
        }
    }
}


impl AigEdge {
    pub fn eval(&self, assgm: &HashMap<String, bool>) -> bool {
        let n: &AigNode = &self.node;
        match n {
            &AigNode::TrueNode => !self.flip,
            &AigNode::VarNode(ref var) => assgm[var],
            &AigNode::AndNode(ref e1, ref e2) => self.flip ^ (e1.eval(assgm) && e2.eval(assgm)),
        }
    }
}

pub fn truen() -> AigEdge {
    AigEdge {
        flip: false,
        node: Rc::new(AigNode::TrueNode),
    }
}

pub fn var(s: String) -> AigEdge {
    AigEdge {
        flip: false,
        node: Rc::new(AigNode::VarNode(s)),
    }
}

pub fn and(a: &AigEdge, b: &AigEdge) -> AigEdge {
    if a.node.as_ref() == &AigNode::TrueNode {
        if !a.flip {
            return b.clone();
        } else {
            return not(&truen());
        }
    }
    if b.node.as_ref() == &AigNode::TrueNode {
        return and(b, a);
    }

    if a.node == b.node {
        if a.flip == b.flip {
            truen()
        } else {
            not(&truen())
        }
    } else {
        AigEdge {
            flip: false,
            node: Rc::new(AigNode::AndNode(a.clone(), b.clone())),
        }
    }
}

pub fn not(a: &AigEdge) -> AigEdge {
    AigEdge {
        flip: !a.flip,
        node: a.node.clone(),
    }
}

pub fn or(a: &AigEdge, b: &AigEdge) -> AigEdge {
    not(&and(&not(a), &not(b)))
}

pub fn eq(a: &AigEdge, b: &AigEdge) -> AigEdge {
    or(&and(a, b), &and(&not(a), &not(b)))
}
