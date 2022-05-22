pub mod ld2il;
pub use crate::ld2il::*;

fn main() {
    println!("Hello, world!");

    let x0 = create_element_node("X0", LdElementKind::Contact);
    let x1 = create_element_node("X1", LdElementKind::Contact);


    let op = create_operation_node(&x0, &x1);

    dbg!(op);

}
