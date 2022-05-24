use core::fmt;
use std::collections::HashSet;

type Id = u32;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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
pub enum AstNodeKind {
    Node(NodeId),
    AstOperation(AstOperationId),
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct AstNodeId { id: Id }
impl From<Id> for AstNodeId {
    fn from(id: Id) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone)]
pub struct AstNode {
    kind: AstNodeKind,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct AstOperationId { id: Id }
impl From<Id> for AstOperationId {
    fn from(id: Id) -> Self {
        Self { id }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AstOperationKind {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct AstOperation {
    pub id: AstOperationId,
    pub kind: AstOperationKind,
    pub op1: AstNodeId,
    pub op2: AstNodeId,
}

#[derive(Debug, Clone)]
pub struct Ladder {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,

    pub ast_nodes: Vec<AstNode>,
    pub ast_operations: Vec<AstOperation>,
}

impl Ladder {
    pub fn new() -> Ladder {
        Ladder {
            //Node is a ladder object, connected to other ladder objects
            nodes: Vec::new(),
            //Connections are between one or more source nodes and one or more sink nodes
            connections: Vec::new(),

            //Ast Nodes are either a ladder node, or an operation on nodes
            ast_nodes: Vec::new(),
            ast_operations: Vec::new(),
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

    pub fn new_ast_node(&mut self, kind: AstNodeKind) -> AstNodeId {
        let id = self.ast_nodes.len() as Id;

        self.ast_nodes.push(
            AstNode {
                kind,
            }
        );

        id.into()
    }

    pub fn new_ast_operation(&mut self, kind: AstOperationKind, op1: AstNodeId, op2: AstNodeId) -> AstNodeId {
        let id = self.ast_operations.len() as Id;

        self.ast_operations.push(
            AstOperation {
                id: id.into(),
                kind,
                op1,
                op2,
            }
        );

        self.new_ast_node_from_operation(id.into())
    }

    fn new_ast_node_from_operation(&mut self, op: AstOperationId) -> AstNodeId {
        self.new_ast_node(
            AstNodeKind::AstOperation(op),
        )
    }

    pub fn print_ast_node(&self, id: AstNodeId) {
        let node = self.ast_nodes[id.id as usize].clone();
        //println!("AstNode: {:?}", node);
        match node.kind {
            AstNodeKind::Node(node_id) => {
                //println!("Node id: {:?}", node_id);
                self.print_node(node_id);
            }
            AstNodeKind::AstOperation(op_id) => {
                //println!("Op id: {:?}", op_id);
                self.print_operation(op_id)
            }
        }
    }

    pub fn print_node(&self, id: NodeId) {
        let node = self.nodes[id.id as usize].clone();
        node.print();
    }

    pub fn print_operation(&self, id: AstOperationId) {
        let node = self.ast_operations[id.id as usize].clone();
        //println!("Operation: {:?}", node);

        self.print_ast_node(node.op1);
        self.print_ast_node(node.op2);

        match node.kind {
            AstOperationKind::And => {
                println!("AND");
            }
            AstOperationKind::Or => {
                println!("OR");
            }
        }
    }

    pub fn generate_ast(&self) -> Result<AstNodeId, ParseError> {


        Err(ParseError::Unknown)
    }
}

#[derive(Debug, Clone)]
pub enum ParseError {
    Unknown,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Unknown => {
                write!(f, "parse error: unknown")
            }
        }
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

