type Id = usize;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeId { pub id: Id }
impl From<Id> for NodeId {
    fn from(id: Id) -> Self {
        Self { id }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum NodeKind {
    Contact,
    Coil,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub label: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct NodePool {
    pub nodes: Vec<Node>,
}

impl NodePool {
    pub fn new() -> NodePool {
        NodePool {
            nodes: Vec::new(),
        }
    }

    pub fn new_node<S>(&mut self, kind: NodeKind, label: S) -> NodeId
    where S: Into<String> {
        let id = self.nodes.len() as Id;

        self.nodes.push(
            Node {
                id: NodeId{ id },
                kind,
                label: label.into(),
            }
        );

        NodeId { id }
    }
}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct AstNodeId { pub id: Id }
impl From<Id> for AstNodeId {
    fn from(id: Id) -> Self {
        Self { id }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum OperationKind {
    And,
    Or,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AstNodeKind {
    Node(NodeId),
    Operation(OperationKind),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct AstNode {
    pub id: AstNodeId,
    pub kind: AstNodeKind,
}

pub struct AstNodePool {
    pub nodes: Vec<AstNode>,
}

impl AstNodePool {
    pub fn new() -> AstNodePool {
        AstNodePool {
            nodes: Vec::new(),
        }
    }

    pub fn new_node(&mut self, kind: AstNodeKind) -> AstNodeId {
        let id = self.nodes.len() as Id;

        self.nodes.push(
            AstNode {
                id: AstNodeId{ id },
                kind,
            }
        );

        AstNodeId { id }
    }
}
