use aig::*;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::cell::RefCell;
use arena::TypedArena;

#[derive(PartialEq, Eq, Hash, Clone)]
enum NodeKey<'a> {
    Input(&'a String),
    And(BTreeSet<AigEdge<'a>>)
}


pub struct Formula<'a> {
    nodes: RefCell<HashMap<NodeKey<'a>, &'a AigNode<'a>>>,
    inputs : RefCell<BTreeSet<&'a String>>,
    truen: AigNode<'a>,
    node_arena : TypedArena<AigNode<'a>>,
    input_arena: TypedArena<String>
}
impl<'a> Formula<'a>{
pub fn new() -> Formula<'a> {
    let na = TypedArena::new();
    let ret = Formula{
        nodes:RefCell::new(HashMap::new()), 
        input_arena: TypedArena::new(),
        node_arena: na,
        inputs: RefCell::new(BTreeSet::new()),
        truen : AigNode::TrueNode};
    //ret.truen = Some(ret.node_arena.alloc(AigNode::TrueNode));
    ret
}

pub fn trueval(&'a self) -> AigEdge<'a> {
    AigEdge{ flip:false, node:&self.truen}
}

pub fn var(&'a self, s: &String) -> AigEdge {
    let mut inps = self.inputs.borrow_mut();
    let mut nodes = self.nodes.borrow_mut();
    let (inp, node): (&String, &'a AigNode<'a>) = match inps.get(s){
        Some(old) => (*old,nodes[&NodeKey::Input(old)]),
        None => {
        let inp = self.input_arena.alloc(s.clone());
        let newnode = self.node_arena.alloc(AigNode::VarNode(inp));
        nodes.insert(NodeKey::Input(inp), newnode);
        (inp, nodes[&NodeKey::Input(inp)])
    }};
    
    inps.insert(inp);

    AigEdge {flip:false, node:node}
}

pub fn not(&'a self, a: &AigEdge<'a>) -> AigEdge<'a> {
    AigEdge {
        flip: !a.flip,
        node: a.node,
    }
}

pub fn and(&'a self, a: &AigEdge<'a>, b: &AigEdge<'a>) -> AigEdge<'a> {
    if a.node == &AigNode::TrueNode {
        if !a.flip {
            return b.clone();
        } else {
            return self.not(&self.trueval());
        }
    }
    if b.node == &AigNode::TrueNode {
        return self.and(b, a);
    }

    if a.node == b.node {
        if a.flip == b.flip {
            self.trueval()
        } else {
            self.not(&self.trueval())
        }
    } else {
        let mut args  =BTreeSet::new();
        args.insert(a.clone());
        args.insert(b.clone());
        let key = NodeKey::And(args);
        let mut nodes = self.nodes.borrow_mut();
        let noderef = if nodes.contains_key(&key) {
            nodes[&key]
        }else {
            let new_node = self.node_arena.alloc(AigNode::AndNode(a.clone(), b.clone()));
            nodes.insert(key.clone(), new_node);
            nodes[&key]
        };
       
        
        AigEdge {
            flip: false,
            node: noderef
        }
    }

}

pub fn or(&'a  self, a: &AigEdge<'a>, b: &AigEdge<'a>) -> AigEdge<'a> {
    let nota = self.not(a);
    let notb = self.not(b);
    let norab = self.and(&nota, &notb);
    self.not(&norab)}

pub fn eq(&'a self, a: &AigEdge<'a>, b: &AigEdge<'a>) -> AigEdge<'a> {
self.or(&self.and(a, b), &self.and(&self.not(a), &self.not(b)))
}
//*/

}