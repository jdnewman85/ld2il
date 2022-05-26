type Id = u32;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeId { id: Id }
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
