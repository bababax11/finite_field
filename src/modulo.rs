use std::ops::Add;
#[derive(Debug, PartialEq)]
pub struct Field {
    v: i32,
    n: i32,
}
impl Field {
    pub fn new(value: i32, n: i32) -> Self {
        Self { v: value, n: n }
    }
}
impl Add for Field {
    type Output = Field;

    fn add(self, other: Field) -> Self {
        assert_eq!(self.n, other.n);
        Self {
            v: (self.v + other.v) % self.n,
            n: self.n,
        }
    }
}
#[test]
fn add() {
    let x = Field::new(1, 3) + Field::new(4, 3);
    assert_eq!(x, Field::new(2, 3));
}
