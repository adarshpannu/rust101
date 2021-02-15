#![allow(warnings)]

use std::cell::RefCell;
use typed_arena::Arena;

#[derive(Debug)]
struct ProjectNode {
    cols: Vec<usize>,
}

#[derive(Debug)]
struct CSVNode {
    name: String,
}

impl CSVNode {
    fn new(arena: &Arena<Node>, name: String) -> &Node {
        let base = NodeBase::CSVNode(CSVNode {
            name: "/tmp/foo.txt".to_string(),
        });
        let node_id = arena.len();

        arena.alloc(Node {
            node_id,
            base,
            source: vec![],
        })
    }
}

#[derive(Debug)]
enum NodeBase {
    CSVNode(CSVNode),
    ProjectNode(ProjectNode),
}

type node_id = usize;

#[derive(Debug)]
struct Node {
    node_id: node_id,
    base: NodeBase,
    source: Vec<node_id>,
}

impl Node {
    fn project<'a>(&self, arena: &'a Arena<Node>, cols: Vec<usize>) -> &'a Node {
        let base = NodeBase::ProjectNode(ProjectNode { cols: cols });
        let node_id = arena.len();

        arena.alloc(Node {
            node_id,
            base,
            source: vec![],
        })
    }
}

struct NodeArena {
    nodes: Vec<Node>,
}

impl NodeArena {
    fn new() -> NodeArena {
        NodeArena { nodes: vec![] }
    }

    fn store_node(&mut self, base: NodeBase, source: Vec<node_id>) -> node_id {
        let node_id = self.nodes.len();
        let node = Node {
            node_id,
            base,
            source,
        };
        self.nodes.push(node);
        node_id
    }

    fn get_node(&self, ix: node_id) -> &Node {
        &self.nodes[ix]
    }
}
struct Monster {
    level: u32,
}

fn main() {
    let arena: Arena<_> = Arena::new();

    let pipeline = CSVNode::new(&arena, "/tmp/foo.csv".to_string())
        .project(&arena, vec![0, 1, 2])
        .project(&arena, vec![2, 1]);

    dbg!(&pipeline);
}
