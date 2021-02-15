#![allow(warnings)]

use std::cell::RefCell;

struct ProjectNode {
    cols: Vec<usize>,
}

struct CSVNode {
    name: String,
}

impl CSVNode {
    fn new(arena: &NodeArena, name: String) -> &Node {
        let base = NodeBase::CSVNode(CSVNode {
            name: "/tmp/foo.txt".to_string(),
        });
        let node_id = arena.store_node(base, vec![]);
        arena.get_node(node_id)
    }
}

enum NodeBase {
    CSVNode(CSVNode),
    ProjectNode(ProjectNode),
}

type node_id = usize;

struct Node {
    node_id: node_id,
    base: NodeBase,
    source: Vec<node_id>,
}

impl Node {
    fn project<'a>(&self, arena: &'a NodeArena, cols: Vec<usize>) -> &'a Node {
        let pnode = NodeBase::ProjectNode(ProjectNode { cols: cols });
        let node_id = arena.store_node(pnode, vec![self.node_id]);
        arena.get_node(node_id)
    }
}

struct NodeArena {
    nodes: RefCell<Vec<Node>>
}

impl NodeArena {
    fn new() -> NodeArena {
        NodeArena { nodes: RefCell::new(vec![]) }
    }

    fn store_node(&self, base: NodeBase, source: Vec<node_id>) -> node_id {
        let node_id = self.nodes.borrow().len();
        let node = Node {
            node_id,
            base,
            source,
        };
        self.nodes.borrow_mut().push(node);
        node_id
    }

    fn get_node(&self, ix: node_id) -> &Node {
        let elems = &*self.nodes.borrow();
        &elems[ix]
    }
}

fn main() {
    let arena = NodeArena::new();

    let csvnode = CSVNode::new(&arena, "/tmp/foo.csv".to_string());

    let pnode = csvnode.project(&arena, vec![0, 2]);
}
