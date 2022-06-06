#![feature(drain_filter)]

/*
 * PROBLEM: I think processing ands is happening in the wrong order
 *   a & b & (c || d) & e is getting turned into: ((a & b) & ((c || d) & e)) instead of
 *                                                (((a & b) & (c || d)) & e)
 */

pub mod ld2il;
pub use crate::ld2il::*;
pub mod util;
pub use crate::util::*;
pub mod reduce;
pub use crate::reduce::*;

use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::stable_graph::StableGraph;

use std::error::Error;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub enum EdgeKind {
    #[default]
    Edge,
    Operands,
}

fn main() -> Result<(), Box<dyn Error>> {
//  println!("Hello, world!");

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

    let mut ld = Graph::<NodeId, EdgeKind>::new();
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
//  let mut ast_ld = Graph::<AstNodeId, ()>::new();
    let ast_ld = ld.map(
        |_, node| {
            ast_node_pool.new_node(AstNodeKind::Node(*node))
        },
        |_, edge| {
            *edge
        },
    );

    let mut ast_ld: StableGraph<AstNodeId, EdgeKind> = ast_ld.into();

    // Reduce
/*
    reduce_into_and(&mut ast_ld, x00_n, x01_n, &mut ast_node_pool);
    reduce_into_and(&mut ast_ld, x05_n, x06_n, &mut ast_node_pool);
    reduce_into_or (&mut ast_ld, x02_n, x03_n, &mut ast_node_pool);
*/


    write_dot_to_png(
        "0.png",
        &format!("{:?}", Dot::new(&ast_ld)),
    );
    // TODO Will probably need a StableGraph to be able to safely bulk modify the graph
//  println!("REDUCE");
    while reduce(&mut ast_ld, &mut ast_node_pool) {}

    // Leaf node filter test
//  let ast_ld = filter_leaf_nodes(&ast_ld, &ast_node_pool);
//
//  dbg!(ast_ld);
//
//  println!("{:?}", Dot::with_config(&ld, &[Config::EdgeNoLabel]));
//  println!("{:?}", Dot::with_config(&ast_ld, &[Config::EdgeNoLabel]));
/*
    write_dot_to_png(
        "test_more.png",
        &format!("{:?}", Dot::with_config(&ast_ld, &[Config::EdgeNoLabel])),
    );
*/


//  topo_print_node_map(&ld);
//  topo_print_ast_node_map(&ast_node_pool, &ast_ld);

    let pretty = ast_ld.map(| _, &ast_node_id| {
        let node_kind = ast_node_pool.nodes[ast_node_id.id].kind;
        match node_kind {
            AstNodeKind::Node(node_id) => node_pool.nodes[node_id.id].label.clone(),
            AstNodeKind::Operation(op_kind) => format!("{:?}", op_kind),
        }
    },
    |_, &edge_id| {
        edge_id
    });
    write_dot_to_png(
        "pretty_end.png",
        &format!("{:?}", Dot::new(&pretty)),
    );

    Ok(())
}

