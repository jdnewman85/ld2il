#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub enum NodeKind {
    #[default]
    None,
    Contact,
    Coil,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub tag: String,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum OperationKind {
    And,
    Or,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub enum AstNodeKind {
    #[default]
    None,
    Node(Node),
    Operation(OperationKind),
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct AstNode {
    pub kind: AstNodeKind,
}
