use std::collections::HashSet;

/*
 * LadderTile
 *   Wire
 *     Horizontal (and)
 *     Vertical   ( or)
 *     Both
 *    Contact
 *      NO
 *      NC
 *    Coil
 * LadderDiagram
 *   [][]LadderTile
 *
 * LadderNode
 *   Contact
 *   Coil
 *   LadderConnection
 *     AndOperation
 *     OrOperation
 *
 *     Sources []LadderNode
 *     Sinks   []LadderNode
 *
 * LadderLogicRung
 *   LadderLogicTree<LadderNode>
 * LadderConnectionDiagram
 *   []LadderRung
 * 
 */

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum NodeKind {
    HWire,
    VWire,
    Contact,
    Coil,
}
pub type NodeId = u32;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub id: NodeId,
    pub label: String,
}


pub type ConnectionId = u32;
#[derive(Debug, Clone)]
pub struct Connection {
    id: ConnectionId,
    pub sources: HashSet<NodeId>,
    pub sinks: HashSet<NodeId>,
}

#[derive(Debug, Copy, Clone)]
pub enum OperationKind {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub kind: OperationKind,
    pub op1: NodeId,
    pub op2: NodeId,
}

#[derive(Debug, Clone)]
pub struct Ladder {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}


impl Ladder {
    pub fn new() -> Ladder {
        Ladder {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn new_node<S>(&mut self, kind: NodeKind, label: S) -> NodeId
    where S: Into<String>,
    {
        let id = self.nodes.len() as u32;

        self.nodes.push(
            Node {
                kind,
                id: id.into(),
                label: label.into(),
            }
        );

        id
    }

    pub fn new_connection(&mut self, sources: HashSet<NodeId>, sinks: HashSet<NodeId>) -> u32
    {
        let id = self.connections.len() as u32;

        self.connections.push(
            Connection { 
                id,
                sources: sources.into(),
                sinks: sinks.into(),
            }
        );

        id
    }
}

