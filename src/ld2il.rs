use std::collections::HashSet;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub enum LdNodeKind {
    #[default]
    Wire,
    Contact,
    Coil,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct LdNode {
    pub label: String,
    pub kind: LdNodeKind,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub enum AstNode {
    #[default]
    None,
    LdNode(LdNode),
    AndOperation,
    OrOperation,
}

#[derive(Debug, Default, Clone)]
pub struct AstNet {
    pub label: String,
    pub sources: HashSet<AstNode>,
    pub sinks: HashSet<AstNode>,
}

pub fn create_ast_node<S>(label: S, kind: LdNodeKind) -> AstNode
where S: Into<String> {
    AstNode::LdNode(
        LdNode{
            label: label.into(),
            kind,
        }
    )
}
pub fn create_ast_net<S>(label: S, sources: Option<Vec<AstNode>>, sinks: Option<Vec<AstNode>>) -> AstNet
where S: Into<String> {
    let mut net = AstNet {
        label: label.into(),
        sources: HashSet::new(),
        sinks: HashSet::new(),
    };

    if let Some(sources) = sources {
        sources.into_iter().for_each(|s| {
            net.sources.insert(s);
        });
    }

    if let Some(sinks) = sinks {
        sinks.into_iter().for_each(|s| {
            net.sinks.insert(s);
        });
    }

    net
}
