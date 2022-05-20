

/* Let's keep track of connections,
 * and base the and/or logic on them
 *
 * Maybe create the connections in a map as needed?
 *
 * Any connection from only two nodes,
 * create AND group
 *
 * For any output nodes of the current connection,
 * add to possibleOr set
 * For each possibleOr's outputConnections,
 * intersect with possibleOr set,
 * Create or out of result (choose simple/complex)
 * 
 */

use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum LdNodeKind {
    Wire,
    Contact,
    Coil,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct LdNode {
    label: String,
    kind: LdNodeKind,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum AstNode {
    LdNode(LdNode),
    AndOperation,
    OrOperation,
}

#[derive(Debug, Clone)]
struct AstNet {
    label: String,
    sources: HashSet<AstNode>,
    sinks: HashSet<AstNode>,
}

fn create_ast_node<S>(label: S, kind: LdNodeKind) -> AstNode
where S: Into<String> {
    AstNode::LdNode(
        LdNode{
            label: label.into(),
            kind,
        }
    )
}
fn create_ast_net<S>(label: S, sources: Option<Vec<AstNode>>, sinks: Option<Vec<AstNode>>) -> AstNet
where S: Into<String> {
    let mut net = AstNet {
        label: label.into(),
        sources: HashSet::new(),
        sinks: HashSet::new(),
    };

    if let Some(sources) = sources {
        sources.into_iter().for_each(|src| {
            net.sources.insert(src);
        });
    }

    if let Some(sinks) = sinks {
        sinks.into_iter().for_each(|snk| {
            net.sinks.insert(snk);
        });
    }

    net
}

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

    dbg!(ast_nets);
}
