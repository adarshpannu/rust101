#![allow(warnings)]

use std::cell::RefCell;
use typed_arena::Arena;

#[derive(Debug)]
struct ProjectNode {
    cols: Vec<usize>,
}

#[derive(Debug)]
struct UnionNode {}

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
    UnionNode(UnionNode),
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

    fn union<'a>(&self, arena: &'a Arena<Node>, sources: Vec<&Node>) -> &'a Node {
        let base = NodeBase::UnionNode(UnionNode {});
        let node_id = arena.len();
        let mut source: Vec<_> = sources.iter().map(|e| e.node_id).collect();
        source.push(self.node_id);

        arena.alloc(Node {
            node_id,
            base,
            source,
        })
    }

    fn name(&self) -> String {
        let nodetype = match &self.base {
            NodeBase::CSVNode(_) => "CSVNode",
            NodeBase::ProjectNode(_) => "ProjectNode",
            NodeBase::UnionNode(_) => "UnionNode",
        };
        format!("{}-{}|{}", nodetype, self.node_id, self.node_id)
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

#[test]
fn test() {
    let arena: Arena<_> = Arena::new();

    /*
                       C ->
                    /      \
        A -> B ->              -> E
                            /
                    \  D ->

    */
    let ab = CSVNode::new(&arena, "/tmp/foo.csv".to_string()).project(&arena, vec![0, 1, 2]);
    let c = ab.project(&arena, vec![0]);
    let d = ab.project(&arena, vec![1]);
    let e = c.union(&arena, vec![d]);

    let arena = arena.into_vec();

    write_pipeline_to_graphviz(
        &arena,
        "/Users/adarshrp/Projects/rust101/src/foo.dot",
        false,
    )
    .expect("Cannot write to .dot file.")
}

use std::io::Write;
use std::process::Command;

fn write_pipeline_to_graphviz(
    arena: &Vec<Node>,
    filename: &str,
    open_jpg: bool,
) -> std::io::Result<()> {
    let mut file = std::fs::File::create(filename)?;
    file.write_all("digraph example1 {\n".as_bytes())?;
    file.write_all("    node [shape=record];\n".as_bytes())?; // data flows up (bottom-to-top)
    file.write_all("    rankdir=LR;\n".as_bytes())?; // data flows up (bottom-to-top)
    file.write_all("    splines=polyline;\n".as_bytes())?; // data flows up (bottom-to-top)
    file.write_all("    nodesep=0.5;\n".as_bytes())?; // data flows up (bottom-to-top)

    for node in arena {
        let nodestr = format!("    Node{}[label=\"{}\"];\n", node.node_id, node.name());
        file.write_all(nodestr.as_bytes())?;

        for source in node.source.iter() {
            let edge = format!("    Node{} -> Node{};\n", source, node.node_id);
            file.write_all(edge.as_bytes())?;
        }
    }
    file.write_all("}\n".as_bytes())?;
    drop(file);

    let ofilename = format!("{}.jpg", filename);
    let oflag = format!("-o{}.jpg", filename);

    // dot -Tjpg -oex.jpg exampl1.dot
    let cmd = Command::new("dot")
        .arg("-Tjpg")
        .arg(oflag)
        .arg(filename)
        .status()
        .expect("failed to execute process");

    if open_jpg {
        let cmd = Command::new("open")
            .arg(ofilename)
            .status()
            .expect("failed to execute process");
    }

    Ok(())
}
