
/*
 * Do I need the ladder object?
 * Could I instead use the underlying collections directly?
 *
 */


pub mod ld2il;
pub use crate::ld2il::*;

use petgraph::graph::Graph;
use petgraph::dot::{Dot, Config};

use std::collections::HashSet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
//    println!("Hello, world!");

    let mut ld = ld2il::Ladder::new();

    let x00 = ld.new_node(ld2il::NodeKind::Contact, "X00");
    let x01 = ld.new_node(ld2il::NodeKind::Contact, "X01");
    let x02 = ld.new_node(ld2il::NodeKind::Contact, "X02");
    let x03 = ld.new_node(ld2il::NodeKind::Contact, "X03");
    let x04 = ld.new_node(ld2il::NodeKind::Contact, "X04");
    let x05 = ld.new_node(ld2il::NodeKind::Contact, "X05");
    let x06 = ld.new_node(ld2il::NodeKind::Contact, "X06");
    let x07 = ld.new_node(ld2il::NodeKind::Contact, "X07");
    let x08 = ld.new_node(ld2il::NodeKind::Contact, "X08");
    let x09 = ld.new_node(ld2il::NodeKind::Contact, "X09");
    let x10 = ld.new_node(ld2il::NodeKind::Contact, "X10");
    let x11 = ld.new_node(ld2il::NodeKind::Contact, "X11");
    let x12 = ld.new_node(ld2il::NodeKind::Contact, "X12");
    let x13 = ld.new_node(ld2il::NodeKind::Contact, "X13");
    let y00 = ld.new_node(ld2il::NodeKind::Coil,    "Y00");
    let y01 = ld.new_node(ld2il::NodeKind::Coil,    "Y01");

    //let l1 =
    ld.new_connection(
        HashSet::new(),
        [x00].into(),
    );
    //let a =
    ld.new_connection(
        [x00].into(),
        [x01].into(),
    );
    //let b =
    ld.new_connection(
        [x01].into(),
        [x02, x03].into(),
    );
    //let c =
    ld.new_connection(
        [x02, x03].into(),
        [x04].into(),
    );
    //let d =
    ld.new_connection(
        [x04].into(),
        [x05, x07].into(),
    );
    //let e =
    ld.new_connection(
        [x05].into(),
        [x06].into(),
    );
    //let f =
    ld.new_connection(
        [x07].into(),
        [x08].into(),
    );
    //let g =
    ld.new_connection(
        [x06, x08].into(),
        [x09].into(),
    );
    //let h =
    ld.new_connection(
        [x09].into(),
        [x13].into(),
    );
    //let i =
    ld.new_connection(
        [x13, x10, x12].into(),
        [y00, y01].into(),
    );
    //let j =
    ld.new_connection(
        [x11].into(),
        [x12].into(),
    );
    //let l2 =
    ld.new_connection(
        [y00, y01].into(),
        HashSet::new(),
    );



    let mut ast = Graph::<NodeId, ()>::new();
    let x00_n = ast.add_node(x00);
    let x01_n = ast.add_node(x01);
    let x02_n = ast.add_node(x02);
    let x03_n = ast.add_node(x03);
    let x04_n = ast.add_node(x04);
    let x05_n = ast.add_node(x05);
    let x06_n = ast.add_node(x06);
    let x07_n = ast.add_node(x07);
    let x08_n = ast.add_node(x08);
    let x09_n = ast.add_node(x09);
    let x10_n = ast.add_node(x10);
    let x11_n = ast.add_node(x11);
    let x12_n = ast.add_node(x12);
    let x13_n = ast.add_node(x13);
    let y00_n = ast.add_node(y00);
    let y01_n = ast.add_node(y01);
    /*
    let a = ast.add_edge(x00_n, x01_n, ());
    let b = ast.add_edge(x00_n, x01_n, ());
    */
    ast.extend_with_edges(&[
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

    println!("{:?}", Dot::with_config(&ast, &[Config::EdgeNoLabel]));


    //dbg!(ld);

    Ok(())
}
