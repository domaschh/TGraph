use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fmt::Display,
    hash::Hash,
    rc::Rc,
    vec,
};

struct Edge {
    weight: RefCell<usize>,
    node: RefCell<Rc<Node>>,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight && **self.node.borrow() == **other.node.borrow()
    }
}

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.weight.borrow().hash(state);
        self.node.borrow().hash(state);
    }
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

fn djiktstra(start_node: Rc<Node>, end_Node: Rc<Node>) -> HashSet<Rc<Node>>{
    let mut prio_queue = PriorityQueue::new();
    let mut shortest_path_table: HashSet<Rc<Node>> = HashSet::new();

    prio_queue.push(start_node, 0);

    while !prio_queue.is_empty() {
        let current_node = prio_queue.pop().expect("Node is empty");
        let currently_conected_nodes = current_node.0.value.borrow();

        if currently_conected_nodes.is_empty() {
            continue;
        }

        for connected_edge in currently_conected_nodes.edges.borrow().iter() {
            let path_to_node = current_node.1 + *connected_edge.weight.borrow();

            if shortest_path_table.contains(&*connected_edge.node.borrow()) {continue;}

            if *connected_edge.node.borrow().value.cost_to_start.borrow() > path_to_node {
                connected_edge
                    .node
                    .borrow()
                    .value
                    .set_cost_to_start(path_to_node);
                connected_edge
                    .node
                    .borrow()
                    .value
                    .set_prev_node(current_node.0.clone());

                prio_queue.push(connected_edge.node.borrow().clone(), *connected_edge.node.borrow().value.cost_to_start.borrow());
            }

            shortest_path_table.insert(current_node.0.clone());
        }
    }

    shortest_path_table
}

#[derive(Hash, PartialEq, Eq)]
struct Node {
    id: usize,
    value: NodeValue,
}

struct NodeValue {
    spf_prev: RefCell<Option<Rc<Node>>>,
    edges: RefCell<Vec<Rc<Edge>>>,
    cost_to_start: RefCell<usize>,
}

impl NodeValue {
    fn set_cost_to_start(&self, new_cost: usize) {
        *self.cost_to_start.borrow_mut() = new_cost;
    }

    fn set_prev_node(&self, prev_node: Rc<Node>) {
        *self.spf_prev.borrow_mut() = Some(prev_node);
    }
}

impl NodeValue {
    fn is_empty(&self) -> bool {
        self.edges.borrow().len() == 0
    }
}

impl PartialEq for NodeValue {
    fn eq(&self, other: &Self) -> bool {
        *self.edges.borrow() == *other.edges.borrow()
    }
}

impl Hash for NodeValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self.edges.borrow()).hash(state);
    }
}

impl Eq for NodeValue {}

impl Node {
    fn add_edge(&self, edge: Rc<Edge>) {
        self.value.edges.borrow_mut().push(edge)
    }

    fn new(id: usize) -> Rc<Self> {
        Rc::new(Node {
            id,
            value: NodeValue {
                spf_prev: RefCell::new(None),
                cost_to_start: RefCell::new(std::usize::MAX),
                edges: RefCell::new(vec![]),
            },
        })
    }

    fn remove_edge(&self, edge: Rc<Edge>) {
        self.value
            .edges
            .borrow_mut()
            .iter()
            .filter(|list_edge| !Rc::ptr_eq(list_edge, &edge));
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node: {}, Connections:[", self.id);
        for edge in self.value.edges.borrow().iter() {
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
    let node3 = Node::new(3);
    let node4 = Node::new(4);

    let edge1 = Edge::new(1, node2.clone());
    let edge2 = Edge::new(1, node3.clone());
    let edge3 = Edge::new(3, node3.clone());
    let edge4 = Edge::new(5, node1.clone());
    let edge5 = Edge::new(2, node1.clone());


    node1.add_edge(edge1);
    node1.add_edge(edge2);

    node2.add_edge(edge3);
    node2.add_edge(edge4)


    // node1.remove_edge(edge1.clone());
    // print!("-----------------------\n");
    // println!("{}", node1);
    // node1.add_edge(edge1.clone());
    // print!("-----------------------\n");
    // println!("{}", node1);
    // edge1.change_node_ptr(node1.clone());
    // println!("{}", node1);
    // print!("-----------------------\n");

    // let vecto = (1..10).collect::<Vec<_>>();
    // vecto
    //     .iter()
    //     .filter(|&&number| number > 5)
    //     .collect::<Vec<_>>();
    // println!("{:?}", vecto);

}
