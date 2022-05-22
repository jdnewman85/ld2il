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

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub enum LdElementKind {
    #[default]
    Wire,
    Contact,
    Coil,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct LdElement {
    pub label: String,
    pub kind: LdElementKind,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub enum OperationKind {
    #[default]
    And,
    Or,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub enum LdNode<'a> {
    #[default]
    None,
    LdElement(LdElement),
    Operation(OperationKind, &'a LdNode<'a>, &'a LdNode<'a>),
}

#[derive(Debug, Default, Clone)]
pub struct LdConnection<'a> {
    pub label: String,
    pub sources: HashSet<LdNode<'a>>,
    pub sinks: HashSet<LdNode<'a>>,
}

pub fn create_operation_node<'a>(op1: &'a LdNode<'a>, op2: &'a LdNode<'a>) -> LdNode<'a> {
    LdNode::Operation(
        OperationKind::And,
        op1,
        op2,
    )
}

pub fn create_element_node<'a, S>(label: S, kind: LdElementKind) -> LdNode<'a>
where S: Into<String> {
    LdNode::LdElement(
        LdElement{
            label: label.into(),
            kind,
        }
    )
}

pub fn create_connection<'a, S>(label: S, sources: Option<Vec<LdNode<'a>>>, sinks: Option<Vec<LdNode<'a>>>) -> LdConnection<'a>
where S: Into<String> {
    let mut con = LdConnection {
        label: label.into(),
        sources: HashSet::new(),
        sinks: HashSet::new(),
    };

    if let Some(sources) = sources {
        sources.into_iter().for_each(|s| {
            con.sources.insert(s);
        });
    }

    if let Some(sinks) = sinks {
        sinks.into_iter().for_each(|s| {
            con.sinks.insert(s);
        });
    }

    con
}
