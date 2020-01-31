use std::ops::{Add, Div, Mul, Neg, Not, Sub};
#[derive(Debug, PartialEq)]
pub struct Field {
    v: i32,
    n: u32,
}
impl Field {
    pub fn new(value: i32, n: u32) -> Self {
        Self { v: value % (n as i32), n: n}
    }
}
impl Neg for Field {
    type Output = Field;
    fn neg(self) -> Self {
        Self {
            v: -self.v,
            n: self.n,
        }
    }
}
impl Not for Field {
    type Output = Field;
    fn not(self) -> Self {
        if self.n == 2 {
            Self {
                v: !self.v as i32,
                n: 2
            }
        } else {
            Self {
                v: self.v.pow((self.n - 2) as u32),
                n: self.n,
            }
        }
    }
}
impl Add for Field {
    type Output = Field;

    fn add(self, other: Field) -> Self {
        assert_eq!(self.n, other.n);
        Self {
            v: (self.v + other.v) % (self.n as i32),
            n: self.n,
        }
    }
}
impl Sub for Field {
    type Output = Field;
    fn sub(self, other: Field) -> Self {
        assert_eq!(self.n, other.n);
        self + (-other)
    }
}
impl Mul for Field {
    type Output = Field;
    fn mul(self, other: Field) -> Self {
        assert_eq!(self.n, other.n);
        Self {
            v: self.v * other.v % (self.n as i32),
            n: self.n,
        }
    }
}
impl Div for Field {
    type Output = Field;
    fn div(self, other: Field) -> Self {
        assert_eq!(self.n, other.n);
        self * (!other)
    }
}
#[test]
fn add() {
    let x = Field::new(1, 3) + Field::new(4, 3);
    assert_eq!(x, Field::new(2, 3));
}
#[test]
fn sub() {
    let x = Field::new(1, 3) - Field::new(4, 3);
    assert_eq!(x, Field::new(0, 3));
}
#[test]
fn mul() {
    let x = Field::new(2, 5) * Field::new(4, 5);
    assert_eq!(x, Field::new(3, 5));
}
#[test]
fn div() {    
    let x = Field::new(2, 5) / Field::new(3, 5);
    assert_eq!(x, Field::new(4, 5));
}