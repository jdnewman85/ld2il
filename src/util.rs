use crate::EdgeKind;
pub use crate::ld2il::*;

use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::visit::DfsPostOrder;
use petgraph::stable_graph::{StableGraph, NodeIndex};
use petgraph::visit::Topo;

use std::{process::{Command, Stdio}, io::Write, path::Path};

#[allow(unused)]
pub fn to_pretty(graph: &StableGraph<AstNode, EdgeKind>) -> StableGraph<String, EdgeKind>
{
    graph.map(| _, ast_node| {
        let node_kind = &ast_node.kind;
        match node_kind {
            AstNodeKind::None => "NONE".into(),
            AstNodeKind::Node(node) => node.tag.clone(),
            AstNodeKind::Operation(op_kind) => format!("{:?}", op_kind),
        }
    },
    |_, &edge_id| {
        edge_id
    })
}

#[allow(unused)]
pub fn write_graph_to_png<N, S>(graph: &StableGraph<N, EdgeKind>, filename: S)
where N: std::fmt::Debug + Clone,
      S: Into<String> + Clone
{
    let dot = &format!("{:?}", Dot::new(&graph));

    let file_string: String = filename.into();
    let out_path = Path::new("./dot/").join(&file_string);
//  println!("Writing dot {:?}", &out_path);
    let mut cmd = Command::new("dot")
        .args(["-T", "png", "-o", &out_path.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .unwrap();

    {
        let mut stdin = cmd.stdin.take().unwrap();
        stdin.write_all(dot.as_bytes()).unwrap();
    }
    cmd.wait();
    //println!("Finished");
}

#[allow(unused)]
pub fn print_il_test<N>(graph: &StableGraph<N, EdgeKind>, starting_output: NodeIndex, s2: NodeIndex)
where N: std::fmt::Debug + Clone
{
    println!("topo_print_il_test--------------------------------");

    let mut visitor = DfsPostOrder::new(graph, starting_output);
    visitor.stack.push(s2);
    while let Some(nx) = visitor.next(graph) {
        let node = graph[nx].clone();
        println!("{:?}", node);
    }
}

#[allow(unused)]
pub fn print_reduced_pretty_il_test(graph: &Graph<String, EdgeKind>, starting_output: NodeIndex, s2: NodeIndex)
{
    let mut g = graph.clone();
    g.reverse();

    print_il_test(&g.into(), starting_output, s2)
}

#[allow(unused)]
pub fn topo_print_node_map<N>(graph: &Graph<N, EdgeKind>)
where N: std::fmt::Debug + Copy
{
    println!("topo_print_node_map--------------------------------");

    let mut topo = Topo::new(graph);
    while let Some(nx) = topo.next(graph) {
        let node = graph[nx];
        println!("\n\nNode: {:?}", node);

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
pub fn topo_print_ast_node_map(graph: &Graph<AstNode, EdgeKind>) {
    println!("topo_print_ast_node_map--------------------------------");

    let mut topo = Topo::new(graph);
    while let Some(nx) = topo.next(graph) {
        let ast_node = &graph[nx];
        println!("\n\nAstNode: {:?}", ast_node);

        if let AstNodeKind::Node(node) = &ast_node.kind {
            println!("\tNodeKind: {:?}", node);
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
