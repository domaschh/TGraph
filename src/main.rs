use std::{
    cell::{Ref, RefCell},
    fmt::Display,
    rc::{Rc, self},
};

struct Edge {
    weight: RefCell<usize>,
    node: RefCell<Rc<Node>>,
}

impl Edge {
    fn new(weight: usize, nodeRef: Rc<Node>) -> Rc<Self> {
        Rc::new(Edge {
            weight: RefCell::new(weight),
            node: RefCell::new(nodeRef),
        })
    }

    fn change_weight(&self, new_weight: usize) {
        self.weight.replace(new_weight);
    }
 
    fn change_node_ptr(&self, new_node: Rc<Node>) {
        self.node.replace(new_node);
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Weight: {} to Node: {})",
            self.weight.borrow(),
            self.node.borrow().id
        )
    }
}

struct Node {
    id: usize,
    edges: RefCell<Vec<Rc<Edge>>>,
}

impl Node {
    fn add_edge(&self, edge: Rc<Edge>) {
        self.edges.borrow_mut().push(edge)
    }

    fn new(id: usize) -> Rc<Self> {
        Rc::new(Node {
            id,
            edges: RefCell::new(vec![]),
        })
    }

    fn remove_edge(&self, edge: Rc<Edge>) {
        self.edges
            .borrow_mut()
            .iter()
            .filter(|listEdge| !Rc::ptr_eq(listEdge, &edge));
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node: {}, Connections:[", self.id);
        for edge in self.edges.borrow().iter() {
            write!(f, "{}", edge);
        }
        println!("");
        write!(f, "]");
        Ok(())
    }
}

fn main() {
    let node1 = Node::new(1);
    let node2 = Node::new(2);

    let edge1 = Edge::new(1, node2.clone());
    let edge2 = Edge::new(2, node1.clone());

    node1.remove_edge(edge1.clone());
    print!("-----------------------\n");
    println!("{}", node1);
    node1.add_edge(edge1.clone());
    print!("-----------------------\n");
    println!("{}", node1);
    edge1.change_node_ptr(node1.clone());
    println!("{}", node1);
    print!("-----------------------\n");
}
