use std::collections::HashSet;

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
    pub kind: NodeKind,
    pub id: NodeId,
    pub label: String,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct ConnectionId { id: Id }
impl From<Id> for ConnectionId {
    fn from(id: Id) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    id: ConnectionId,
    pub sources: HashSet<NodeId>,
    pub sinks: HashSet<NodeId>,
}

#[derive(Debug, Clone)]
pub struct Ladder {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,

}

impl Ladder {
    pub fn new() -> Ladder {
        Ladder {
            //Node is a ladder object, connected to other ladder objects
            nodes: Vec::new(),
            //Connections are between one or more source nodes and one or more sink nodes
            connections: Vec::new(),
        }
    }

    pub fn new_node<S>(&mut self, kind: NodeKind, label: S) -> NodeId
    where S: Into<String>,
    {
        let id = self.nodes.len() as Id;

        self.nodes.push(
            Node {
                kind,
                id: id.into(),
                label: label.into(),
            }
        );

        id.into()
    }

    pub fn new_connection(&mut self, sources: HashSet<NodeId>, sinks: HashSet<NodeId>) -> ConnectionId
    {
        let id = self.connections.len() as Id;

        self.connections.push(
            Connection {
                id: id.into(),
                sources: sources.into(),
                sinks: sinks.into(),
            }
        );

        id.into()
    }

    pub fn print_node(&self, id: NodeId) {
        let node = self.nodes[id.id as usize].clone();
        node.print();
    }
}

impl Node {
    pub fn print(&self) {
        //println!("Node: {:?}", self);
        match self.kind {
            NodeKind::Contact => {
                //println!("Contact:");
                println!("PUSH {}", self.label);
            }
            NodeKind::Coil => {
                //println!("Coil:");
                println!("OUT {}", self.label);
            }
        }
    }
}
