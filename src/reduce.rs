pub use crate::ld2il::*;

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;

use std::collections::HashSet;

//TODO Generalize this and the or?
pub fn reduce_into_and(graph: &mut Graph<AstNodeId, ()>, lh_node: NodeIndex, rh_node: NodeIndex, node_pool: &mut AstNodePool) {
    //TODO Return and node?
    //TODO Return Result<>

    //TODO Ensure Node1 sinks are only connected to node2 source
    //       Single sink/source, and edges_connecting also single

    // Store sources from lh_node, and sinks from rh_node
    let lh_sources: Vec<NodeIndex> = graph.neighbors_directed(lh_node, petgraph::Incoming).collect();
    let rh_sinks: Vec<NodeIndex>   = graph.neighbors_directed(rh_node, petgraph::Outgoing).collect();

    // Remove nodes and store weights
    let lh_weight = graph.remove_node(lh_node).unwrap();
    let rh_weight = graph.remove_node(rh_node).unwrap();

    // Create AND node
    let and0 = node_pool.new_node(AstNodeKind::Operation(OperationKind::And));
    let and0_n = graph.add_node(and0);
    // Create new nodes for lh, an rh
    let new_lh_node = graph.add_node(lh_weight);
    let new_rh_node = graph.add_node(rh_weight);

    // Connect lh and rh nodes to new AND node
    graph.extend_with_edges(&[
        (new_lh_node, and0_n),
        (new_rh_node, and0_n),
    ]);

    //Reconnect edges based on what existed prior
    for src_node in lh_sources {
        graph.add_edge(src_node, and0_n, ());
    }
    for sink_node in rh_sinks {
        graph.add_edge(and0_n, sink_node, ());
    }
}

pub fn reduce_into_or(graph: &mut Graph<AstNodeId, ()>, lh_node: NodeIndex, rh_node: NodeIndex, node_pool: &mut AstNodePool) {
    //TODO Return and node?
    //TODO Return Result<>

    //TODO Ensure node sinks and sources match exactly

    // Store sources from lh_node, and sinks from rh_node
    let lh_sources: HashSet<NodeIndex> = graph.neighbors_directed(lh_node, petgraph::Incoming).collect();
    let rh_sinks:   HashSet<NodeIndex> = graph.neighbors_directed(rh_node, petgraph::Outgoing).collect();

    // Remove nodes and store weights
    let lh_weight = graph.remove_node(lh_node).unwrap();
    let rh_weight = graph.remove_node(rh_node).unwrap();

    // Create OR node
    let or0 = node_pool.new_node(AstNodeKind::Operation(OperationKind::Or));
    let or0_n = graph.add_node(or0);
    // Create new nodes for lh, an rh
    let new_lh_node = graph.add_node(lh_weight);
    let new_rh_node = graph.add_node(rh_weight);

    // Connect lh and rh nodes to new AND node
    graph.extend_with_edges(&[
        (new_lh_node, or0_n),
        (new_rh_node, or0_n),
    ]);

    //Reconnect edges based on what existed prior
    for src_node in lh_sources {
        graph.add_edge(src_node, or0_n, ());
    }
    for sink_node in rh_sinks {
        graph.add_edge(or0_n, sink_node, ());
    }
}

/*
fn reduce_ast(mut graph: &Graph<AstNodeId, ()>, ast_node_pool:AstNodePool ) {

    todo!()
}
 */
