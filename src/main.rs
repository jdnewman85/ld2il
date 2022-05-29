
/*
 * Need to copy->convert the ld into ast ladder, before starting to reduce!
 */

pub mod ld2il;
pub use crate::ld2il::*;
pub mod util;
pub use crate::util::*;

use petgraph::graph::Graph;
use petgraph::dot::{Dot, Config};
use petgraph::graph::NodeIndex;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
//    println!("Hello, world!");

    let mut node_pool = ld2il::NodePool::new();
    let x00 = node_pool.new_node(ld2il::NodeKind::Contact, "X00");
    let x01 = node_pool.new_node(ld2il::NodeKind::Contact, "X01");
    let x02 = node_pool.new_node(ld2il::NodeKind::Contact, "X02");
    let x03 = node_pool.new_node(ld2il::NodeKind::Contact, "X03");
    let x04 = node_pool.new_node(ld2il::NodeKind::Contact, "X04");
    let x05 = node_pool.new_node(ld2il::NodeKind::Contact, "X05");
    let x06 = node_pool.new_node(ld2il::NodeKind::Contact, "X06");
    let x07 = node_pool.new_node(ld2il::NodeKind::Contact, "X07");
    let x08 = node_pool.new_node(ld2il::NodeKind::Contact, "X08");
    let x09 = node_pool.new_node(ld2il::NodeKind::Contact, "X09");
    let x10 = node_pool.new_node(ld2il::NodeKind::Contact, "X10");
    let x11 = node_pool.new_node(ld2il::NodeKind::Contact, "X11");
    let x12 = node_pool.new_node(ld2il::NodeKind::Contact, "X12");
    let x13 = node_pool.new_node(ld2il::NodeKind::Contact, "X13");
    let y00 = node_pool.new_node(ld2il::NodeKind::Coil,    "Y00");
    let y01 = node_pool.new_node(ld2il::NodeKind::Coil,    "Y01");

    let mut ld = Graph::<NodeId, ()>::new();
    let x00_n = ld.add_node(x00);
    let x01_n = ld.add_node(x01);
    let x02_n = ld.add_node(x02);
    let x03_n = ld.add_node(x03);
    let x04_n = ld.add_node(x04);
    let x05_n = ld.add_node(x05);
    let x06_n = ld.add_node(x06);
    let x07_n = ld.add_node(x07);
    let x08_n = ld.add_node(x08);
    let x09_n = ld.add_node(x09);
    let x10_n = ld.add_node(x10);
    let x11_n = ld.add_node(x11);
    let x12_n = ld.add_node(x12);
    let x13_n = ld.add_node(x13);
    let y00_n = ld.add_node(y00);
    let y01_n = ld.add_node(y01);

    ld.extend_with_edges(&[
        (x00_n, x01_n),
        (x01_n, x02_n), (x01_n, x03_n),
        (x02_n, x04_n), (x03_n, x04_n),
        (x04_n, x05_n), (x04_n, x07_n),
        (x05_n, x06_n),
        (x07_n, x08_n),
        (x06_n, x09_n), (x08_n, x09_n),
        (x06_n, x10_n), (x08_n, x10_n),
        (x06_n, x11_n), (x08_n, x11_n),
        (x09_n, x13_n),
        (x10_n, y00_n), (x10_n, y01_n),
        (x11_n, x12_n),
        (x13_n, y00_n), (x13_n, y01_n),
        (x12_n, y00_n), (x12_n, y01_n),
    ]);


    // Convert
    let mut ast_node_pool = ld2il::AstNodePool::new();
    //let mut ast_ld = Graph::<AstNodeId, ()>::new();
    let mut ast_ld = ld.map(
        |_, node| {
            ast_node_pool.new_node(AstNodeKind::Node(*node))
        },
        |_, edge| {
            *edge
        },
    );

    // Reduce
    reduce_into_and(&mut ast_ld, x00_n, x01_n, &mut ast_node_pool);
    reduce_into_and(&mut ast_ld, x05_n, x06_n, &mut ast_node_pool);

    //dbg!(ast_ld);
    //TODO factor out into function

    //println!("{:?}", Dot::with_config(&ld, &[Config::EdgeNoLabel]));
    println!("{:?}", Dot::with_config(&ast_ld, &[Config::EdgeNoLabel]));

//    topo_print_node_map(&ld);
//    topo_print_ast_node_map(&ast_node_pool, &ast_ld);

    Ok(())
}

fn reduce_into_and(graph: &mut Graph<AstNodeId, ()>, lh_node: NodeIndex, rh_node: NodeIndex, node_pool: &mut AstNodePool) {
    //TODO Return and node?
    //TODO Return Result<>

    // Ensure Node1 sinks are only connected to node2 source
    //   Single sink/source, and edges_connecting also single

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

/*
fn reduce_ast(mut graph: &Graph<AstNodeId, ()>, ast_node_pool:AstNodePool ) {

    todo!()
}
 */
