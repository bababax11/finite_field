use super::manipulative::Manipulative;
use super::modulo::Field;
fn euclid_decrypt(p: u32, n: usize, d: usize, a: i32, y: Vec<i32>) -> Manipulative<Field> {
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
        let (q, _r_0) = r_m1.divide_by(&r_0).unwrap();
        r_m1 = std::mem::replace(&mut r_0, _r_0);
        let _t_0 = t_0.clone();
        t_m1 = std::mem::replace(&mut t_0, t_m1.clone() - q * _t_0);
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
    y -= Manipulative::new(err);
    y
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decryption_test() {
        let w = euclid_decrypt(5, 4, 3, 2, vec![1, 0, 2, 1]);
        assert_eq!(
            w,
            Manipulative::new([4, 0, 2, 1].iter().map(|v| Field::new(*v, 5)).collect())
        );
    }
}
