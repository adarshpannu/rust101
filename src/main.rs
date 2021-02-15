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
            source: vec![self.node_id],
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

    /*
                       C ->
                    /      \
        A -> B ->              -> E
                            /
                    \  D ->

    */
    let ab = CSVNode::new(&arena, "/tmp/foo.csv".to_string())
        .project(&arena, vec![0, 1, 2]);
    let c = ab.project(&arena, vec![0]);
    let d = ab.project(&arena, vec![1]);

    dbg!(&ab);

    let arena = arena.into_vec();
    for node in arena {
        dbg!(node);
    }
}

#[test]
fn test() {
    use id_arena::{Arena, Id};

    type AstNodeId = Id<AstNode>;

    #[derive(Debug, Eq, PartialEq)]
    pub enum AstNode {
        Const(i64),
        Var(String),
        Add { lhs: AstNodeId, rhs: AstNodeId },
        Sub { lhs: AstNodeId, rhs: AstNodeId },
        Mul { lhs: AstNodeId, rhs: AstNodeId },
        Div { lhs: AstNodeId, rhs: AstNodeId },
    }

    let mut ast_nodes = Arena::<AstNode>::new();

    // Create the AST for `a * (b + 3)`.
    let three = ast_nodes.alloc(AstNode::Const(3));
    let b = ast_nodes.alloc(AstNode::Var("b".into()));
    let b_plus_three = ast_nodes.alloc(AstNode::Add { lhs: b, rhs: three });
    let a = ast_nodes.alloc(AstNode::Var("a".into()));
    let a_times_b_plus_three = ast_nodes.alloc(AstNode::Mul {
        lhs: a,
        rhs: b_plus_three,
    });

    // Can use indexing to access allocated nodes.
    assert_eq!(ast_nodes[three], AstNode::Const(3));
}
