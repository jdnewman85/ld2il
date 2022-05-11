

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
    in_nodes: HashSet<AstNode>,
    out_nodes: HashSet<AstNode>,
}

fn create_ast_net<S>(label: S) -> AstNet
where S: Into<String> {
    AstNet{
        label: label.into(),
        in_nodes: HashSet::new(),
        out_nodes: HashSet::new(),
    }
}

fn main() {
    println!("Hello, world!");

    let mut ast_nets = Vec::new();

    let node_0 = AstNode::LdNode(
        LdNode{
            label: "node_0".to_string(),
            kind: LdNodeKind::Contact,
        },
    );

    let in_nodes = HashSet::new();
    let mut out_nodes = HashSet::new();
    out_nodes.insert(node_0);

    let net_0 = AstNet{
        label: "net_0".to_string(),
        in_nodes,
        out_nodes,
    };

    ast_nets.push(net_0);

    let node_1 = AstNode::LdNode(
        LdNode{
            label: "node_1".to_string(),
            kind: LdNodeKind::Contact,
        }
    );
    let mut net_1 = create_ast_net("net_1");
    net_1.out_nodes.insert(node_1);
    ast_nets.push(net_1);

    dbg!(ast_nets);
}
