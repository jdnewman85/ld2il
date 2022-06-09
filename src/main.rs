#![feature(drain_filter)]

pub mod ld2il;
pub use crate::ld2il::*;
pub mod util;
pub use crate::util::*;
pub mod reduce;
pub use crate::reduce::*;

use petgraph::dot::Dot;
use petgraph::graph::Graph;

use std::error::Error;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub enum EdgeKind {
    #[default]
    Edge,
    Operands,
}

fn main() -> Result<(), Box<dyn Error>> {
    let x00 = Node{kind: ld2il::NodeKind::Contact, tag: "X00".into()};
    let x01 = Node{kind: ld2il::NodeKind::Contact, tag: "X01".into()};
    let x02 = Node{kind: ld2il::NodeKind::Contact, tag: "X02".into()};
    let x03 = Node{kind: ld2il::NodeKind::Contact, tag: "X03".into()};
    let x04 = Node{kind: ld2il::NodeKind::Contact, tag: "X04".into()};
    let x05 = Node{kind: ld2il::NodeKind::Contact, tag: "X05".into()};
    let x06 = Node{kind: ld2il::NodeKind::Contact, tag: "X06".into()};
    let x07 = Node{kind: ld2il::NodeKind::Contact, tag: "X07".into()};
    let x08 = Node{kind: ld2il::NodeKind::Contact, tag: "X08".into()};
    let x09 = Node{kind: ld2il::NodeKind::Contact, tag: "X09".into()};
    let x10 = Node{kind: ld2il::NodeKind::Contact, tag: "X10".into()};
    let x11 = Node{kind: ld2il::NodeKind::Contact, tag: "X11".into()};
    let x12 = Node{kind: ld2il::NodeKind::Contact, tag: "X12".into()};
    let x13 = Node{kind: ld2il::NodeKind::Contact, tag: "X13".into()};
    let y00 = Node{kind: ld2il::NodeKind::Coil,    tag: "Y00".into()};
    let y01 = Node{kind: ld2il::NodeKind::Coil,    tag: "Y01".into()};

    let mut ld = Graph::<Node, EdgeKind>::new();
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
    let ast_ld = ld.map(
        |_, node| {
            AstNode{ kind: AstNodeKind::Node(node.clone()) }
        },
        |_, &edge| {
            edge
        },
    );

    let mut ast_ld: LdAstGraph = ast_ld.into();

    write_dot_to_png(
        "0.png",
        &format!("{:?}", Dot::new(&ast_ld)),
    );
    while reduce(&mut ast_ld) {}

    let pretty = ast_ld.map(| _, ast_node| {
        let node_kind = &ast_node.kind;
        match node_kind {
            AstNodeKind::None => "NONE".into(),
            AstNodeKind::Node(node) => node.tag.clone(),
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

    let pretty: Graph<_, _> = pretty.into();

    let mut pretty = pretty.clone();
    pretty.reverse();

    print_il_test(&pretty.into(), y00_n, y01_n);

    Ok(())
}

