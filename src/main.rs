extern crate finite_field;
use finite_field::decryption::euclid_decrypt;
use finite_field::manipulative::Manipulative;
use finite_field::modulo::Field;
fn main() {
    // 整式の割り算
    let a = Manipulative::new(vec![1, 2, 1]);
    let b = Manipulative::new(vec![1, 1]);
    assert_eq!(
        a.divide_by(&b).unwrap(),
        (
            Manipulative::new(vec![1, 1, 0]),
            Manipulative::new(vec![0, 0, 0])
        )
    );

    // 過去問の問題のユークリッド復号
    let w = euclid_decrypt(11, 7, 5, 2, &[1, -1, 1, 0, 3, 2, 0, 1]);
    println!("{:?}", w);
    assert_eq!(
        w,
        Manipulative::new(
            [1, 10, 1, 8, 3, 2, 5, 1]
                .iter()
                .map(|v| Field::new(*v, 11))
                .collect()
        )
    );
}
