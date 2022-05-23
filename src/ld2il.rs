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

#[derive(Debug, Copy, Clone)]
pub enum ElementKind {
    HWire,
    VWire,
    Contact,
    Coil,
}
type ElementId = String;

#[derive(Debug, Clone)]
pub struct Element {
    pub kind: ElementKind,
    pub id: ElementId,
}


type ConnectionId = u32;
type HashSet_ElementId = HashSet<ElementId>;
#[derive(Debug, Clone)]
pub struct Connection {
    id: ConnectionId,
    pub sources: HashSet_ElementId,
    pub sinks: HashSet_ElementId,
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

    pub fn new_element<S>(&mut self, kind: ElementKind, id: S)
    where S: Into<ElementId>
    {
        self.elements.push(
            Element {
                kind,
                id: id.into(),
            }
        );
    }

    pub fn new_connection<S>(&mut self, sources: S, sinks: S) -> u32
    where S: Into<HashSet_ElementId>
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

