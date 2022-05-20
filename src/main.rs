use maplit::hashset;

pub mod ld2il;
pub use crate::ld2il::*;

fn main() {
    println!("Hello, world!");

    let mut ast_nets = Vec::new();

    ast_nets.push(
        create_ast_net(
            "net_0",
            Some(vec![create_ast_node("node_0", LdNodeKind::Contact)]),
            None,
        )
    );

    ast_nets.push(
        create_ast_net(
            "net_1",
            Some(vec![create_ast_node("node_1", LdNodeKind::Contact)]),
            None,
        )
    );

    ast_nets.push(
        AstNet {
            label: "net_2".into(),
            sinks: hashset!{create_ast_node("node_2", LdNodeKind::Contact)},
            ..Default::default()
        }
    );

    dbg!(ast_nets);
}
