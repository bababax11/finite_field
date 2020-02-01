use std::ops;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Field {
    pub v: i32,
    pub n: u32,
}
impl Field {
    pub fn new(value: i32, n: u32) -> Self {
        Self {
            v: value % (n as i32),
            n: n,
        }
    }
}
impl ops::Neg for Field {
    type Output = Field;
    fn neg(self) -> Self {
        Self {
            v: (self.n as i32 -self.v) % self.n as i32,
            n: self.n,
        }
    }
}
impl ops::Not for Field {
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
impl ops::Add for Field {
    type Output = Field;

    fn add(self, other: Field) -> Self {
        assert_eq!(self.n, other.n);
        Self {
            v: (self.v + other.v) % (self.n as i32),
            n: self.n,
        }
    }
}
impl ops::Add<i32> for Field {
    type Output = Field;

    fn add(self, other: i32) -> Self {
        Self {
            v: (self.v + other) % (self.n as i32),
            n: self.n,
        }
    }
}
impl ops::AddAssign for Field {
    fn add_assign(&mut self, other: Field) {
        assert_eq!(self.n, other.n);
        self.v = (self.v + other.v) % (self.n as i32);
    }
}
impl ops::Sub for Field {
    type Output = Field;
    fn sub(self, other: Field) -> Self {
        assert_eq!(self.n, other.n);
        Self {
            v: (self.v - other.v + self.n as i32) % (self.n as i32),
            n: self.n,
        }
    }
}
impl ops::SubAssign for Field {
    fn sub_assign(&mut self, other: Field) {
        assert_eq!(self.n, other.n);
        self.v = (self.v - other.v + self.n as i32) % (self.n as i32);
    }
}
impl ops::Mul for Field {
    type Output = Field;
    fn mul(self, other: Field) -> Self {
        assert_eq!(self.n, other.n);
        Self {
            v: self.v * other.v % (self.n as i32),
            n: self.n,
        }
    }
}
impl ops::Mul<i32> for Field {
    type Output = Field;

    fn mul(self, other: i32) -> Self {
        Self {
            v: (self.v * other) % (self.n as i32),
            n: self.n,
        }
    }
}
impl ops::MulAssign for Field {
    fn mul_assign(&mut self, other: Field) {
        assert_eq!(self.n, other.n);
        self.v = (self.v * other.v) % (self.n as i32);
    }
}
impl ops::MulAssign<i32> for Field {
    fn mul_assign(&mut self, other: i32) {
        self.v = (self.v * other) % (self.n as i32);
    }
}
impl ops::Div for Field {
    type Output = Field;
    fn div(self, other: Field) -> Self {
        assert_eq!(self.n, other.n);
        self * (!other)
    }
}
impl ops::DivAssign for Field {
    fn div_assign(&mut self, other: Field) {
        assert_eq!(self.n, other.n);
        *self *= !other;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let mut x = Field::new(1, 3) + Field::new(4, 3);
        assert_eq!(x, Field::new(2, 3));
        x += x.clone();
        assert_eq!(x, Field::new(1, 3));
    }
    #[test]
    fn sub_test() {
        let x = Field::new(2, 3) - Field::new(4, 3);
        assert_eq!(x, Field::new(1, 3));
    }
    #[test]
    fn mul_test() {
        let x = Field::new(2, 5) * Field::new(4, 5);
        assert_eq!(x, Field::new(3, 5));
    }
    #[test]
    fn div_test() {
        let mut x = Field::new(2, 5) / Field::new(3, 5);
        assert_eq!(x, Field::new(4, 5));
        x /= x.clone();
        assert_eq!(x, Field::new(1, 5));
    }
    #[test]
    fn not_test() {
        let x = !Field::new(1, 2);
        assert_eq!(x, Field::new(0, 2));
        assert_eq!(!x, Field::new(1, 2));
    }
    #[test]
    fn neg_test() {
        let x = -Field::new(2, 3);
        assert_eq!(x, Field::new(1, 3));
    }
}
