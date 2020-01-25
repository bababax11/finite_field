extern crate finite_field;
use finite_field::modulo::Modulo;
fn main() {
    let x = Modulo::new(1, 4);
    println!("{:?}", &x);
}
