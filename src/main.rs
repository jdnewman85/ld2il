pub mod ld2il;
pub use crate::ld2il::*;

use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    let mut ld = ld2il::Ladder::new();

    let x00 = ld.new_element(ElementKind::Contact, "X00");
    let x01 = ld.new_element(ElementKind::Contact, "X01");
    let x02 = ld.new_element(ElementKind::Contact, "X02");
    let x03 = ld.new_element(ElementKind::Contact, "X03");
    let x04 = ld.new_element(ElementKind::Contact, "X04");
    let x05 = ld.new_element(ElementKind::Contact, "X05");
    let x06 = ld.new_element(ElementKind::Contact, "X06");
    let x07 = ld.new_element(ElementKind::Contact, "X07");
    let x08 = ld.new_element(ElementKind::Contact, "X08");
    let x09 = ld.new_element(ElementKind::Contact, "X09");
    let x10 = ld.new_element(ElementKind::Contact, "X10");
    let x11 = ld.new_element(ElementKind::Contact, "X11");
    let x12 = ld.new_element(ElementKind::Contact, "X12");
    let x13 = ld.new_element(ElementKind::Contact, "X13");
    let y00 = ld.new_element(ElementKind::Coil,    "Y00");
    let y01 = ld.new_element(ElementKind::Coil,    "Y01");

    //let l1 =
    ld.new_connection(
        HashSet::new(),
        [x00].into(),
    );
    //let a =
    ld.new_connection::<HashSet<ElementId>>(
        [x00].into(),
        [x01].into(),
    );
    //let b =
    ld.new_connection::<HashSet<ElementId>>(
        [x01].into(),
        [x02, x03].into(),
    );
    //let c =
    ld.new_connection::<HashSet<ElementId>>(
        [x02, x03].into(),
        [x04].into(),
    );
    //let d =
    ld.new_connection::<HashSet<ElementId>>(
        [x04].into(),
        [x05, x07].into(),
    );
    //let e =
    ld.new_connection::<HashSet<ElementId>>(
        [x05].into(),
        [x06].into(),
    );
    //let f =
    ld.new_connection::<HashSet<ElementId>>(
        [x07].into(),
        [x08].into(),
    );
    //let g =
    ld.new_connection::<HashSet<ElementId>>(
        [x06, x08].into(),
        [x09].into(),
    );
    //let h =
    ld.new_connection::<HashSet<ElementId>>(
        [x09].into(),
        [x13].into(),
    );
    //let i =
    ld.new_connection::<HashSet<ElementId>>(
        [x13, x10, x12].into(),
        [y00, y01].into(),
    );
    //let j =
    ld.new_connection::<HashSet<ElementId>>(
        [x11].into(),
        [x12].into(),
    );
    //let l2 =
    ld.new_connection::<HashSet<ElementId>>(
        [y00, y01].into(),
        HashSet::new(),
    );

    dbg!(ld);
}
