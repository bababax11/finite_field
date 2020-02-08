use super::manipulative::Manipulative;
use super::modulo::Field;

/// ユークリッド復号する.
/// p: 体の標数, n: 多項式の最大次数 + 1; C={f(a^i) (0<=i<=d-1) = 0}を線形符号とする.
/// y: 復号したい多項式のスライス(次数の低い順)
/// p: 素数, aの位数の判定は行わない.
/// # Panics
/// `2 <= d <= n` でないとき
/// # Example
/// ```
/// # use finite_field::manipulative::Manipulative;
/// # use finite_field::modulo::Field;
/// # use finite_field::decryption::euclid_decrypt;
/// let w = euclid_decrypt(5, 4, 3, 2, &[1, 0, 2, 1]); // 1 + 2 x^2 + x^3
/// assert_eq!(
///     w,
///     Manipulative::new([4, 0, 2, 1].iter().map(|v| Field::new(*v, 5)).collect()) // 4 + 2 x^2 + x^3
///);
/// ```
pub fn euclid_decrypt(p: u32, n: usize, d: usize, a: i32, y: &[i32]) -> Manipulative<Field> {
    if 2 > d || d > n {
        panic!("dの範囲が2 <= d <= n でない")
    }
    let y = y.iter().map(|v| Field::new(*v, p)).collect();
    let mut y = Manipulative::new(y);

    let mut t_m1 = Manipulative::new(vec![Field::new(0, p)]);
    let mut t_0 = Manipulative::new(vec![Field::new(1, p)]);

    let mut r_m1 = vec![Field::new(0, p); d - 1];
    r_m1.push(Field::new(1, p));
    let mut r_m1 = Manipulative::new(r_m1);

    let mut r_0 = Vec::with_capacity(d - 2);
    let mut _a = Field::new(1, p);
    for _ in 0..d - 1 {
        _a *= a;
        r_0.push(y.assign(_a));
    }
    let mut r_0 = Manipulative::new(r_0);

    if r_0.deg() == -1 {
        return y;
    }
    loop {
        let (q, new_r_0) = r_m1.divide_by(&r_0).unwrap();
        r_m1 = std::mem::replace(&mut r_0, new_r_0);
        let new_t_0 = t_m1.clone() - &(&q * &t_0);
        t_m1 = std::mem::replace(&mut t_0, new_t_0);
        if r_0.deg() <= (d as i32 - 1) / 2 - 1 {
            break;
        }
    }
    let mut _a = Field::new(a, p);
    let a_inv = !Field::new(a, p);
    let mut err = vec![Field::new(0, p); n];
    for e in &mut err {
        _a *= a_inv;
        if t_0.assign(_a) == Field::new(0, p) {
            *e -= r_0.assign(_a) / t_0.diff().assign(_a)
        }
    }
    y -= &Manipulative::new(err);
    y
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decryption_test() {
        let w = euclid_decrypt(5, 4, 3, 2, &[1, 0, 2, 1]);
        assert_eq!(
            w,
            Manipulative::new([4, 0, 2, 1].iter().map(|v| Field::new(*v, 5)).collect())
        );
        let w = euclid_decrypt(5, 4, 3, 2, &[4, 0, 2, 1]);
        assert_eq!(
            w,
            Manipulative::new([4, 0, 2, 1].iter().map(|v| Field::new(*v, 5)).collect())
        );
        let w = euclid_decrypt(11, 7, 5, 2, &[1, -1, 1, 0, 3, 2, 0, 1]);
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
}
