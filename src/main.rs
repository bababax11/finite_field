extern crate finite_field;
use finite_field::modulo::Field;
fn main() {
    let x = Field::new(1, 4);
    println!("{:?}", &x);
}
