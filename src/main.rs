use std::{cell::{RefCell, Ref}, rc::Rc, fmt::{Display}};

type NodeRef = Rc<RefCell<Edge>>;

struct Edge {
    weight: RefCell<usize>,
    node: Rc<Node>
}

impl Edge {
    fn new(weight: usize, nodeRef: Rc<Node>) -> Rc<Self> {
        Rc::new(Edge {weight: RefCell::new(weight), node: nodeRef})
    }    

    fn change_weight(&self, new_weight: usize) {
        self.weight.replace(new_weight);
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"(Weight: {} to Node: {})", self.weight.borrow(), self.node.id)
    }
}

struct Node {
    id: usize,
    edges: RefCell<Vec<Rc<Edge>>>
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Node: {}", self.id);
        for edge in self.edges.borrow().iter(){
            write!(f, "{}", edge);
        }
        Ok(())
    }
}

impl Node {
    fn new(id: usize) -> Self {
        Node {
            id, 
            edges: RefCell::new(vec![])
        }
    }

    fn addEdge(&self, edge: Rc<Edge>) {
        self.edges.borrow_mut().push(edge)
    }

    fn replaceEgde(&self, edge: Edge) {

    }
}

fn main() {
    let node1 = Rc::new(Node::new(1));
    let node2 = Rc::new(Node::new(2));
    
    let edge1 = Edge::new(1, node2.clone());
    let edge2 = Edge::new(2, node1.clone());
    
    node2.addEdge(edge2.clone());
    node1.addEdge(edge1.clone());
    
    println!("{}", node1);
    println!("{}", node1);
    edge1.change_weight(5);

    println!("{}", node1);
    println!("{}", node2);

}
