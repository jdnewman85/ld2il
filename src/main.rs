pub mod ld2il;
pub use crate::ld2il::*;

use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let mut ld = ld2il::Ladder::new();

    let x00 = ld.new_node(NodeKind::Contact, "X00");
    let x01 = ld.new_node(NodeKind::Contact, "X01");
    let x02 = ld.new_node(NodeKind::Contact, "X02");
    let x03 = ld.new_node(NodeKind::Contact, "X03");
    let x04 = ld.new_node(NodeKind::Contact, "X04");
    let x05 = ld.new_node(NodeKind::Contact, "X05");
    let x06 = ld.new_node(NodeKind::Contact, "X06");
    let x07 = ld.new_node(NodeKind::Contact, "X07");
    let x08 = ld.new_node(NodeKind::Contact, "X08");
    let x09 = ld.new_node(NodeKind::Contact, "X09");
    let x10 = ld.new_node(NodeKind::Contact, "X10");
    let x11 = ld.new_node(NodeKind::Contact, "X11");
    let x12 = ld.new_node(NodeKind::Contact, "X12");
    let x13 = ld.new_node(NodeKind::Contact, "X13");
    let y00 = ld.new_node(NodeKind::Coil,    "Y00");
    let y01 = ld.new_node(NodeKind::Coil,    "Y01");

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

    let ast_node_x00 = ld.new_ast_node(
        AstNodeKind::Node(x00)
    );
    let ast_node_x01 = ld.new_ast_node(
        AstNodeKind::Node(x01)
    );
    let and0 =
    ld.new_ast_operation(AstOperationKind::And, ast_node_x00, ast_node_x01);
    let ast_and0 =
    ld.new_ast_node_from_operation(and0);

    let ast_node_x03 = ld.new_ast_node(
        AstNodeKind::Node(x03)
    );
    let ast_node_x04 = ld.new_ast_node(
        AstNodeKind::Node(x04)
    );
    let or0 =
    ld.new_ast_operation(AstOperationKind::Or, ast_node_x03, ast_node_x04);
    let ast_or0 =
    ld.new_ast_node_from_operation(or0);

    let and1 =
    ld.new_ast_operation(AstOperationKind::Or, ast_and0, ast_or0);
    let ast_and1 = ld.new_ast_node_from_operation(and1);

    dbg!(ld);
}
