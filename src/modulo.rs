use std::ops::{Add, Div, Mul, Neg, Not, Sub};
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Field {
    v: i32,
    n: u32,
}
impl Field {
    pub fn new(value: i32, n: u32) -> Self {
        Self {
            v: value % (n as i32),
            n: n,
        }
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
                v: (self.v == 0) as i32,
                n: 2,
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
        Self {
            v: (self.v - other.v) % (self.n as i32),
            n: self.n,
        }
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let x = Field::new(1, 3) + Field::new(4, 3);
        assert_eq!(x, Field::new(2, 3));
    }
    #[test]
    fn sub_test() {
        let x = Field::new(1, 3) - Field::new(4, 3);
        assert_eq!(x, Field::new(0, 3));
    }
    #[test]
    fn mul_test() {
        let x = Field::new(2, 5) * Field::new(4, 5);
        assert_eq!(x, Field::new(3, 5));
    }
    #[test]
    fn div_test() {
        let x = Field::new(2, 5) / Field::new(3, 5);
        assert_eq!(x, Field::new(4, 5));
    }
    #[test]
    fn neg_test() {
        let x = !Field::new(1, 2);
        assert_eq!(x, Field::new(0, 2));
        assert_eq!(!x, Field::new(1, 2));
    }
}
