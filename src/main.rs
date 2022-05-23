pub mod ld2il;
pub use crate::ld2il::*;

use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let mut ld = ld2il::Ladder::new();

    let x0 = ld.new_element(ElementKind::Contact, "X0");
    let x1 = ld.new_element(ElementKind::Contact, "X1");
    let y1 = ld.new_element(ElementKind::Coil, "Y0");

    ld.new_connection(
        HashSet::new(),
        [x0, x1].into(),
    );
    ld.new_connection(
        [x0, x1].into(),
        HashSet::new(),
    );

    dbg!(ld);
}
