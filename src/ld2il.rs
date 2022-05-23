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
type ElementId = String;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Element {
    pub kind: ElementKind,
    pub id: ElementId,
}


type ConnectionId = u32;
#[derive(Debug, Clone)]
pub struct Connection {
    id: ConnectionId,
    pub sources: HashSet<Element>,
    pub sinks: HashSet<Element>,
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
    pub connections: Vec<Connection>,
}

pub fn new_element<S>(kind: ElementKind, id: S) -> Element
where S: Into<ElementId>
{
    Element {
        kind,
        id: id.into(),
    }
}

impl Ladder {
    pub fn new() -> Ladder {
        Ladder {
            connections: Vec::new(),
        }
    }


    pub fn new_connection<S>(&mut self, sources: S, sinks: S) -> u32
    where S: Into<HashSet<Element>>
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

