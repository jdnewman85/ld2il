pub mod ld2il;
pub use crate::ld2il::*;

use maplit::hashset;

use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let mut ld = ld2il::Ladder::new();

    let x0 = ld.new_element(ElementKind::Contact, "X0");
    let x1 = ld.new_element(ElementKind::Contact, "X1");
    let y1 = ld.new_element(ElementKind::Coil, "Y0");
    dbg!(x0, x1, y1);

    let con_0 = ld.new_connection(
        HashSet::new(),
        hashset!["X0".into(), "X1".into()],
    );
    let con_1 = ld.new_connection(
        hashset!["X0".into(), "X1".into()],
        HashSet::new(),
    );

    dbg!(con_0, con_1);

    dbg!(ld);
}
