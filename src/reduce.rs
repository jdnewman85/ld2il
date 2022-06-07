use crate::EdgeKind;
use crate::ld2il::*;
use crate::write_dot_to_png;

use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use petgraph::visit::EdgeRef;
use petgraph::visit::Topo;
use petgraph::dot::Dot;

use std::collections::HashSet;

pub type LdAstGraph = StableGraph<AstNodeId, EdgeKind>;

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

//  dbg!(&lh_sources, &lh_sinks, &rh_sources, &rh_sinks);

    if lh_sinks.len() == 1 && lh_sinks[0] == rh_node {
        if rh_sources.len() == 1  && rh_sources[0] == lh_node {
            return true
        }
    }

    let lh_node_id = graph[lh_node];
    let rh_node_id = graph[rh_node];
    if lh_node_id.id == 16 as usize || lh_node_id.id as usize == 18 && rh_node_id.id as usize == 16 || rh_node_id.id as usize == 18 {
//      println!("Can't AND {:?} with {:?}", lh_node_id, rh_node_id);
//      println!("lh.len() - {} | rh.len() - {}", lh_sinks.len(), rh_sources.len());

        let temp: Vec<_> = graph.edges_directed(rh_node, petgraph::Incoming)
/*
        .filter(|edge_ref| {
            match edge_ref.weight() {
                EdgeKind::Edge => true,
                EdgeKind::Operands => false,
            }
        })
*/
        .map(|edge_ref| {
            graph[edge_ref.source()]
        }).collect();
        let temp2: Vec<_> = graph.edges_directed(rh_node, petgraph::Incoming)
/*
        .filter(|edge_ref| {
            match edge_ref.weight() {
                EdgeKind::Edge => true,
                EdgeKind::Operands => false,
            }
        })
*/
        .collect();
 
//      dbg!(temp, temp2);
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

//  dbg!(&lh_sources, &lh_sinks, &rh_sources, &rh_sinks);
//  dbg!(&source_common, &sink_common);

    if source_common.len() >= 1 {
        if sink_common.len() >= 1 {
            return true
        }
    }

/*
    let lh_node_id = graph[lh_node];
    let rh_node_id = graph[rh_node];
    println!("Can't OR {:?} with {:?}", lh_node_id, rh_node_id);
*/
    return false
}

// TODO Generalize this and the or?
#[allow(unused)]
pub fn reduce_into_and(graph: &mut LdAstGraph, lh_node: NodeIndex, rh_node: NodeIndex, node_pool: &mut AstNodePool) {
    // TODO Return new resulting node?
    // TODO Return Result<>

    // Single sink/source, and edges_connecting also single
    if !nodes_could_and(&graph, lh_node, rh_node) {
        panic!("But you can't AND those!");
    }

    let lh_node_id = graph[lh_node];
    let lh_ast_node = &node_pool.nodes[lh_node_id.id].clone();
    let rh_node_id = graph[rh_node];
    let rh_ast_node = &node_pool.nodes[rh_node_id.id].clone();
//  println!("Creating AND - {:?} -> {:?}", lh_node_id, rh_node_id);

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
        graph[op]
    }).collect();
//  dbg!(&lh_operands_ids);

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
        graph[op]
    }).collect();
//  dbg!(&rh_operands_ids);

    let lh_sources: Vec<NodeIndex> = lh_source_edges.iter().map(|edge_ref| {
        edge_ref.source()
    }).collect();
    let rh_sinks:   Vec<NodeIndex> = graph.neighbors_directed(rh_node, petgraph::Outgoing).collect();

    // Remove nodes and store weights
    let lh_weight = graph.remove_node(lh_node).unwrap();
    let rh_weight = graph.remove_node(rh_node).unwrap();

    // Create AND node
    let and0 = &node_pool.new_node(AstNodeKind::Operation(OperationKind::And));
    let and0_n = graph.add_node(*and0);
    // Create new nodes for lh, an rh
    let new_lh_node = graph.add_node(lh_weight);
    let new_rh_node = graph.add_node(rh_weight);

    // Connect lh and rh nodes to new AND node
    let new_lh_node_id = graph[new_lh_node];
    let new_rh_node_id = graph[new_rh_node];
    let and0_id = graph[and0_n];
//  println!("Attaching lh and rh to and: {:?} & {:?} -> {:?}", new_lh_node_id, new_rh_node_id, and0_id);
    graph.extend_with_edges(&[
        (new_lh_node, and0_n, EdgeKind::Operands),
        (new_rh_node, and0_n, EdgeKind::Operands),
    ]);

    for op_node in lh_operands {
        let op_node_id = graph[op_node];
        let new_node_id = graph[new_lh_node];
//      println!("Attaching lh_operands: {:?} -> {:?}", op_node_id, new_node_id);
        graph.add_edge(op_node, new_lh_node, EdgeKind::Operands);
    }
    for op_node in rh_operands {
        let op_node_id = graph[op_node];
        let new_node_id = graph[new_rh_node];
//      println!("Attaching rh_operands: {:?} -> {:?}", op_node_id, new_node_id);
        graph.add_edge(op_node, new_rh_node, EdgeKind::Operands);
    }

    // Reconnect edges based on what existed prior
    for src_node in lh_sources {
        graph.add_edge(src_node, and0_n, EdgeKind::Edge);
    }
    for sink_node in rh_sinks {
        graph.add_edge(and0_n, sink_node, EdgeKind::Edge);
    }

    write_dot_to_png(
        &format!("{:?} {:?}-{:?}_and.png", and0.id, lh_ast_node.id, rh_ast_node.id),
        &format!("{:?}", Dot::new(&*graph)),
    );
}

#[allow(unused)]
pub fn reduce_into_or(graph: &mut LdAstGraph, lh_node: NodeIndex, rh_node: NodeIndex, node_pool: &mut AstNodePool) {
    // TODO Return new resulting node?
    // TODO Return Result<>

    // Single sink/source, and edges_connecting also single
    if !nodes_could_or(&graph, lh_node, rh_node) {
        panic!("But you can't OR those!");
    }

    let lh_node_id = graph[lh_node];
    let lh_ast_node = &node_pool.nodes[lh_node_id.id].clone();
    let rh_node_id = graph[rh_node];
    let rh_ast_node = &node_pool.nodes[rh_node_id.id].clone();
//  println!("Creating OR - {:?} -> {:?}", lh_node_id, rh_node_id);

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
        graph[op]
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
        graph[op]
    }).collect();
//  dbg!(&rh_operands_ids);

    let lh_sources: Vec<NodeIndex> = lh_source_edges.iter().map(|edge_ref| {
        edge_ref.source()
    }).collect();
    let rh_sinks:   Vec<NodeIndex> = graph.neighbors_directed(rh_node, petgraph::Outgoing).collect();

    // Remove nodes and store weights
    let lh_weight = graph.remove_node(lh_node).unwrap();
    let rh_weight = graph.remove_node(rh_node).unwrap();

    // Create AND node
    let or0 = &node_pool.new_node(AstNodeKind::Operation(OperationKind::Or));
    let or0_n = graph.add_node(*or0);
    // Create new nodes for lh, an rh
    let new_lh_node = graph.add_node(lh_weight);
    let new_rh_node = graph.add_node(rh_weight);

    // Connect lh and rh nodes to new AND node
    let new_lh_node_id = graph[new_lh_node];
    let new_rh_node_id = graph[new_rh_node];
    let or0_id = graph[or0_n];
//  println!("Attaching lh and rh to or: {:?} & {:?} -> {:?}", new_lh_node_id, new_rh_node_id, or0_id);
    graph.extend_with_edges(&[
        (new_lh_node, or0_n, EdgeKind::Operands),
        (new_rh_node, or0_n, EdgeKind::Operands),
    ]);

    for op_node in lh_operands {
        let op_node_id = graph[op_node];
        let new_node_id = graph[new_lh_node];
//      println!("Attaching lh_operands: {:?} -> {:?}", op_node_id, new_node_id);
        graph.add_edge(op_node, new_lh_node, EdgeKind::Operands);
    }
    for op_node in rh_operands {
        let op_node_id = graph[op_node];
        let new_node_id = graph[new_rh_node];
//      println!("Attaching rh_operands: {:?} -> {:?}", op_node_id, new_node_id);
        graph.add_edge(op_node, new_rh_node, EdgeKind::Operands);
    }

    // Reconnect edges based on what existed prior
    for src_node in lh_sources {
        graph.add_edge(src_node, or0_n, EdgeKind::Edge);
    }
    for sink_node in rh_sinks {
        graph.add_edge(or0_n, sink_node, EdgeKind::Edge);
    }

    write_dot_to_png(
        &format!("{:?} {:?}-{:?}_or.png", or0.id, lh_ast_node.id, rh_ast_node.id),
        &format!("{:?}", Dot::new(&*graph)),
    );
}

#[allow(unused)]
//Returns whether or not an operations was performed, which can be used to reduce to completion
pub fn reduce(mut graph: &mut LdAstGraph, mut node_pool: &mut AstNodePool) -> bool {
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
                    reduce_into_or(&mut graph, lh_node, sibling, &mut node_pool);
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
                    reduce_into_and(&mut graph, lh_node, child, &mut node_pool);
                    return true
                }
            }
        }
    }

    false
}
