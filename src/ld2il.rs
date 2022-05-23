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
pub enum ElementKind {
    HWire,
    VWire,
    Contact,
    Coil,
}
type ElementId = u32;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Element {
    pub kind: ElementKind,
    pub id: ElementId,
    pub label: String,
}


type ConnectionId = u32;
#[derive(Debug, Clone)]
pub struct Connection {
    id: ConnectionId,
    pub sources: HashSet<ElementId>,
    pub sinks: HashSet<ElementId>,
}

#[derive(Debug, Copy, Clone)]
pub enum OperationKind {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub kind: OperationKind,
    pub op1: ElementId,
    pub op2: ElementId,
}

#[derive(Debug, Clone)]
pub struct Ladder {
    pub elements: Vec<Element>,
    pub connections: Vec<Connection>,
}


impl Ladder {
    pub fn new() -> Ladder {
        Ladder {
            elements: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn new_element<S>(&mut self, kind: ElementKind, label: S) -> ElementId
    where S: Into<String>,
    {
        let id = self.elements.len() as u32;

        self.elements.push(
            Element {
                kind,
                id: id.into(),
                label: label.into(),
            }
        );

        id
    }

    pub fn new_connection<T>(&mut self, sources: T, sinks: T) -> u32
    where T: Into<HashSet<ElementId>>
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

