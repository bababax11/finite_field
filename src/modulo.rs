use std::ops::Add;
#[derive(Debug, PartialEq)]
pub struct Modulo {
    v: i32,
    n: i32,
}
impl Modulo {
    pub fn new(value: i32, n: i32) -> Self {
        Self { v: value, n: n }
    }
}
impl Add for Modulo {
    type Output = Modulo;

    fn add(self, other: Modulo) -> Self {
        assert_eq!(self.n, other.n);
        Self {
            v: (self.v + other.v) % self.n,
            n: self.n,
        }
    }
}
#[test]
fn add() {
    let x = Modulo::new(1, 3) + Modulo::new(4, 3);
    assert_eq!(x, Modulo::new(2, 3));
}
