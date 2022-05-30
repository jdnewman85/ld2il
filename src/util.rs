pub use crate::ld2il::*;

use petgraph::graph::Graph;
use petgraph::visit::Topo;

//TODO Generalize over nodes?
#[allow(unused)]
fn topo_print_node_map<N>(graph: &Graph<N, ()>) 
where N: std::fmt::Debug + Copy
{
    println!("topo_print_node_map--------------------------------");

    let mut bfs = Topo::new(graph);
    while let Some(nx) = bfs.next(graph) {
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
fn topo_print_ast_node_map(node_pool: &AstNodePool, graph: &Graph<AstNodeId, ()>) {
    println!("topo_print_ast_node_map--------------------------------");

    let mut bfs = Topo::new(graph);
    while let Some(nx) = bfs.next(graph) {
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
pub fn filter_leaf_nodes(graph: &Graph<AstNodeId, ()>, node_pool: &AstNodePool) -> Graph<AstNodeId, ()> {
    //Filter out leaf nodes for ast structure only
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
