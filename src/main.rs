
/*
 * Need to copy->convert the ld into ast ladder, before starting to reduce!
 */


pub mod ld2il;
pub use crate::ld2il::*;

use petgraph::graph::Graph;
use petgraph::dot::{Dot, Config};
use petgraph::visit::Topo;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
//    println!("Hello, world!");

    let mut node_pool = ld2il::NodePool::new();
    let mut ast_node_pool = ld2il::AstNodePool::new();
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


    //Reduce 
    let mut ast_ld = Graph::<AstNodeId, ()>::new();

    //dbg!(ld);
    let a = ld.remove_node(x00_n).unwrap();
    let a = ast_node_pool.new_node(AstNodeKind::Node(a));
    let b = ld.remove_node(x01_n).unwrap();
    let b = ast_node_pool.new_node(AstNodeKind::Node(b));
    let and0 = ast_node_pool.new_node(AstNodeKind::Operation(OperationKind::And));
    let and0_n = ast_ld.add_node(and0);
    let ast_and0_n = ast_ld.add_node(and0);
    let a_n = ast_ld.add_node(a);
    let b_n = ast_ld.add_node(b);
    ast_ld.extend_with_edges(&[
        (a_n, ast_and0_n),
        (b_n, ast_and0_n),
    ]);
    ld.add_edge(and0_n, x03_n, ());

    println!("{:?}", Dot::with_config(&ld, &[Config::EdgeNoLabel]));

    let mut bfs = Topo::new(&ld);
    while let Some(nx) = bfs.next(&ld) {
        let ast_node_id = ld[nx];
        let ast_node = &ast_node_pool.nodes[ast_node_id.id];
        println!("\n\nAstNode: {:?}", ast_node);

        if let AstNodeKind::Node(node_id) = ast_node.kind {
            println!("\tNode: {:?}", node_pool.nodes[node_id.id]);
        }

        for edge in ld.edges_directed(nx, petgraph::Incoming) {
            println!("In Edge: {:?}", edge);
        }
        println!("");
        for edge in ld.edges_directed(nx, petgraph::Outgoing) {
            println!("Out Edge: {:?}", edge);
        }
    }

    let mut bfs = Topo::new(&ast_ld);
    while let Some(nx) = bfs.next(&ast_ld) {
        let ast_node_id = ast_ld[nx];
        let ast_node = &ast_node_pool.nodes[ast_node_id.id];
        println!("\n\nAstNode: {:?}", ast_node);

        if let AstNodeKind::Node(node_id) = ast_node.kind {
            println!("\tNode: {:?}", node_pool.nodes[node_id.id]);
        }

        for edge in ast_ld.edges_directed(nx, petgraph::Incoming) {
            println!("In Edge: {:?}", edge);
        }
        println!("");
        for edge in ast_ld.edges_directed(nx, petgraph::Outgoing) {
            println!("Out Edge: {:?}", edge);
        }
    }
    Ok(())
}

fn reduce_ast(mut graph: &Graph<AstNodeId, ()>, ast_node_pool:AstNodePool ) {
/*
 * 
 */

    todo!()
}
