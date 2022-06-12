use crate::EdgeKind;
use crate::ld2il::*;
use crate::write_graph_to_png;

use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::Topo;
use petgraph::visit::EdgeRef;

use std::collections::HashSet;

pub type LdAstGraph = StableGraph<AstNode, EdgeKind>;

#[allow(unused)]
pub fn nodes_could_and(graph: &LdAstGraph, lh_node: NodeIndex, rh_node: NodeIndex) -> bool {
//  let lh_sources: Vec<NodeIndex> = graph.neighbors_directed(lh_node, petgraph::Incoming).collect();
    let lh_sinks:   Vec<NodeIndex> = graph.neighbors_directed(lh_node, petgraph::Outgoing).collect();
//  let rh_sources: Vec<NodeIndex> = graph.neighbors_directed(rh_node, petgraph::Incoming).collect();
    let rh_sources: Vec<_> = graph.edges_directed(rh_node, petgraph::Incoming)
    .filter(|edge_ref| {
        match edge_ref.weight() {
            EdgeKind::Edge => true,
            EdgeKind::Operands => false,
        }
    })
    .map(|edge_ref| {
        edge_ref.source()
    }).collect();
//  let rh_sinks:   Vec<NodeIndex> = graph.neighbors_directed(rh_node, petgraph::Outgoing).collect();

    if lh_sinks.len() == 1 && lh_sinks[0] == rh_node {
        if rh_sources.len() == 1  && rh_sources[0] == lh_node {
            return true
        }
    }

    return false
}

#[allow(unused)]
pub fn nodes_could_or(graph: &LdAstGraph, lh_node: NodeIndex, rh_node: NodeIndex) -> bool {
    let lh_sources: HashSet<NodeIndex> = graph.neighbors_directed(lh_node, petgraph::Incoming).collect();
    let lh_sinks:   HashSet<NodeIndex> = graph.neighbors_directed(lh_node, petgraph::Outgoing).collect();
    let rh_sources: HashSet<NodeIndex> = graph.neighbors_directed(rh_node, petgraph::Incoming).collect();
    let rh_sinks:   HashSet<NodeIndex> = graph.neighbors_directed(rh_node, petgraph::Outgoing).collect();

    let source_common: Vec<&NodeIndex> = lh_sources.intersection(&rh_sources).collect();
    let sink_common:   Vec<&NodeIndex> = lh_sinks.intersection(&rh_sinks).collect();

    if source_common.len() >= 1 {
        if sink_common.len() >= 1 {
            return true
        }
    }

    return false
}
#[allow(unused)]
pub fn reduce_into_operation(graph: &mut LdAstGraph, lh_node: NodeIndex, rh_node: NodeIndex, op_kind: OperationKind) {
    // TODO Return new resulting node?
    // TODO Return Result<>

    // Single sink/source, and edges_connecting also single
//  TODO Replace with check passed in?
/*
    if !nodes_could_and(&graph, lh_node, rh_node) {
        panic!("But you can't AND those!");
    }
*/

    let lh_ast_node = graph[lh_node].clone();
    let rh_ast_node = graph[rh_node].clone();

    // Store sources from lh_node, and sinks from rh_node
    let mut lh_source_edges: Vec<_> = graph.edges_directed(lh_node, petgraph::Incoming).collect();
    let lh_operands: Vec<_> = lh_source_edges.drain_filter(|edge_ref| {
        match edge_ref.weight() {
            EdgeKind::Edge => false,
            EdgeKind::Operands => true,
        }
    })
    .map(|edge_ref| {
        edge_ref.source()
    }).collect();

    let lh_operands_ids: Vec<_> = lh_operands.iter().map(|&op| {
        graph[op].clone()
    }).collect();

    let mut rh_source_edges: Vec<_> = graph.edges_directed(rh_node, petgraph::Incoming).collect();
    let rh_operands: Vec<_> = rh_source_edges.drain_filter(|edge_ref| {
        match edge_ref.weight() {
            EdgeKind::Edge => false,
            EdgeKind::Operands => true,
        }
    })
    .map(|edge_ref| {
        edge_ref.source()
    }).collect();

    let rh_operands_ids: Vec<_> = rh_operands.iter().map(|&op| {
        graph[op].clone()
    }).collect();

    let lh_sources: Vec<NodeIndex> = lh_source_edges.iter().map(|edge_ref| {
        edge_ref.source()
    }).collect();
    let rh_sinks:   Vec<NodeIndex> = graph.neighbors_directed(rh_node, petgraph::Outgoing).collect();

    // Remove nodes and store weights
    let lh_weight = graph.remove_node(lh_node).unwrap();
    let rh_weight = graph.remove_node(rh_node).unwrap();

    // Create AND node
    let op_ast_node = &AstNode{ kind: AstNodeKind::Operation(op_kind) };
    let op_node = graph.add_node(op_ast_node.clone());
    // Create new nodes for lh, an rh
    let new_lh_node = graph.add_node(lh_weight);
    let new_rh_node = graph.add_node(rh_weight);

    // Connect lh and rh nodes to new AND node
    graph.extend_with_edges(&[
        (new_lh_node, op_node, EdgeKind::Operands),
        (new_rh_node, op_node, EdgeKind::Operands),
    ]);

    for node in lh_operands {
        graph.add_edge(node, new_lh_node, EdgeKind::Operands);
    }
    for node in rh_operands {
        graph.add_edge(node, new_rh_node, EdgeKind::Operands);
    }

    // Reconnect edges based on what existed prior
    for node in lh_sources {
        graph.add_edge(node, op_node, EdgeKind::Edge);
    }
    for node in rh_sinks {
        graph.add_edge(op_node, node, EdgeKind::Edge);
    }

    let filename = &format!("{:?} {:?}-{:?}_and.png", op_ast_node, lh_ast_node, rh_ast_node);
    write_graph_to_png(graph, filename);
}

#[allow(unused)]
//Returns whether or not an operations was performed, which can be used to reduce to completion
pub fn reduce(mut graph: &mut LdAstGraph) -> bool {
    let mut topo_walker = Topo::new(&*graph);
    while let Some(lh_node) = topo_walker.next(&*graph) {
        let lh_sources = graph.neighbors_directed(lh_node, petgraph::Incoming);
        let lh_siblings = lh_sources.fold(Vec::<NodeIndex>::new(),|mut siblings, parent| {
            let mut new_siblings: Vec<NodeIndex> = graph.neighbors_directed(parent, petgraph::Outgoing).collect();
            siblings.append(&mut new_siblings);
            siblings
        });

        for sibling in lh_siblings {
            if lh_node != sibling {
                if nodes_could_or(&graph, lh_node, sibling) {
                    reduce_into_operation(&mut graph, lh_node, sibling, OperationKind::Or);
                    return true
                }
            }
        }
        let lh_sinks = graph.neighbors_directed(lh_node, petgraph::Outgoing);
        let lh_children = lh_sinks.fold(Vec::new(),|mut children, child| {
            children.push(child);
            children
        });

        for child in lh_children {
            if lh_node != child {
                if nodes_could_and(&graph, lh_node, child) {
                    reduce_into_operation(&mut graph, lh_node, child, OperationKind::And);
                    return true
                }
            }
        }
    }

    false
}
