extern crate finite_field;
use finite_field::modulo::Field;
use finite_field::manipulative::Manipulative;
fn main() {
    let x = Field::new(2, 4);
    println!("{:?}", x * Field::new(2, 4));
    let a = Manipulative::new(vec![1, 2, 1]);
        let b = Manipulative::new(vec![1, 1]);
        assert_eq!(
            a.divide_by(&b),
            (
                Manipulative::new(vec![1, 1, 0]),
                Manipulative::new(vec![0, 0, 0])
            )
        );
}
