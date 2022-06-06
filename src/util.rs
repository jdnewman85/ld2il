use crate::EdgeKind;
pub use crate::ld2il::*;

use petgraph::graph::Graph;
use petgraph::visit::Topo;

use std::{process::{Command, Stdio}, io::Write, path::Path};

#[allow(unused)]
pub fn write_dot_to_png<S>(filename: S, dot: S)
where S: Into<String> + Clone
{
    let file_string: String = filename.into();
    let out_path = Path::new("./dot/").join(&file_string);
    println!("Writing dot {:?}", &out_path);
    let mut cmd = Command::new("dot")
        .args(["-T", "png", "-o", &out_path.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .unwrap();

    {
        let mut stdin = cmd.stdin.take().unwrap();
        stdin.write_all(dot.into().as_bytes()).unwrap();
    }
    cmd.wait();
    println!("Finished");
}

// TODO Generalize over nodes?
#[allow(unused)]
pub fn topo_print_node_map<N>(graph: &Graph<N, EdgeKind>)
where N: std::fmt::Debug + Copy
{
    println!("topo_print_node_map--------------------------------");

    let mut topo = Topo::new(graph);
    while let Some(nx) = topo.next(graph) {
        let node_id = graph[nx];
        println!("\n\nNode Id: {:?}", node_id);

        for edge in graph.edges_directed(nx, petgraph::Incoming) {
            println!("In Edge: {:?}", edge);
        }
        println!("");
        for edge in graph.edges_directed(nx, petgraph::Outgoing) {
            println!("Out Edge: {:?}", edge);
        }
    }
}

#[allow(unused)]
pub fn topo_print_ast_node_map(node_pool: &AstNodePool, graph: &Graph<AstNodeId, EdgeKind>) {
    println!("topo_print_ast_node_map--------------------------------");

    let mut topo = Topo::new(graph);
    while let Some(nx) = topo.next(graph) {
        let ast_node_id = graph[nx];
        let ast_node = &node_pool.nodes[ast_node_id.id];
        println!("\n\nAstNode: {:?}", ast_node);

        if let AstNodeKind::Node(node_id) = ast_node.kind {
            println!("\tNode: {:?}", node_pool.nodes[node_id.id]);
        }

        for edge in graph.edges_directed(nx, petgraph::Incoming) {
            println!("In Edge: {:?}", edge);
        }
        println!("");
        for edge in graph.edges_directed(nx, petgraph::Outgoing) {
            println!("Out Edge: {:?}", edge);
        }
    }
}

#[allow(unused)]
pub fn filter_leaf_nodes(graph: &Graph<AstNodeId, EdgeKind>, node_pool: &AstNodePool) -> Graph<AstNodeId, EdgeKind> {
    // Filter out leaf nodes for ast structure only
    graph.filter_map(
        |_, node| {
            let n = &node_pool.nodes[node.id];
            match n.kind {
                AstNodeKind::Node(_node_id) => {
                    None
                },
                AstNodeKind::Operation(_op_kind) => {
                    Some(*node)
                },
            }
        },
        |_, edge| {
            Some(*edge)
        }
    )
}
