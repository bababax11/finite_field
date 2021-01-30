use std::ops;
use typenum::{Unsigned};
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Field<N: Unsigned> {
    pub v: i64,
}
impl<N: Unsigned> Field<N> {
    pub fn new(value: i64) -> Self {
        Self {
            v: value % N::to_i64(),
        }
    }
    fn n(&self) -> u64 {
        N::to_u64()
    }
}
impl<N: Unsigned> ops::Neg for Field<N> {
    type Output = Field<N>;
    fn neg(self) -> Self {
        Self {
            v: (self.n() as i64 - self.v) % self.n() as i64,
        }
    }
}
impl<N: Unsigned> ops::Not for Field<N> {
    type Output = Field<N>;
    fn not(self) -> Self {
        if self.n() == 2 {
            Self {
                v: (self.v == 0) as i64,
            }
        } else {
            Self {
                v: pow(self.v, (self.n() - 2) as u64, self.n() as i64),
            }
        }
    }
}
#[inline]
fn pow(mut base: i64, mut exp: u64, p: i64) -> i64 {
    let mut acc = 1;

    while exp > 1 {
        if (exp & 1) == 1 {
            acc = acc * base % p;
        }
        exp /= 2;
        base = base * base % p;
    }
    if exp == 1 {
        acc = acc * base % p;
    }
    acc
}
impl<N: Unsigned> ops::Add for Field<N> {
    type Output = Field<N>;

    fn add(self, other: Field<N>) -> Self {
        // assert_eq!(self.n, other.n);
        Self {
            v: (self.v + other.v) % (self.n() as i64),
        }
    }
}
impl<N: Unsigned> ops::Add<i64> for Field<N> {
    type Output = Field<N>;

    fn add(self, other: i64) -> Self {
        Self {
            v: (self.v + other) % (self.n() as i64),
        }
    }
}
impl<N: Unsigned> ops::AddAssign for Field<N> {
    fn add_assign(&mut self, other: Field<N>) {
        // assert_eq!(self.n, other.n);
        self.v = (self.v + other.v) % (self.n() as i64);
    }
}
impl<N: Unsigned> ops::Sub for Field<N> {
    type Output = Field<N>;
    fn sub(self, other: Field<N>) -> Self {
        // assert_eq!(self.n, other.n);
        Self {
            v: (self.v - other.v + self.n() as i64) % (self.n() as i64),
            n: self.n,
        }
    }
}
impl<N: Unsigned> ops::SubAssign for Field<N> {
    fn sub_assign(&mut self, other: Field<N>) {
        // assert_eq!(self.n, other.n);
        self.v = (self.v - other.v + self.n() as i64) % (self.n() as i64);
    }
}
impl<N: Unsigned> ops::Mul for Field<N> {
    type Output = Field<N>;
    fn mul(self, other: Field<N>) -> Self {
        // assert_eq!(self.n, other.n);
        Self {
            v: self.v * other.v % (self.n() as i64),
        }
    }
}
impl<N: Unsigned> ops::Mul<i64> for Field<N> {
    type Output = Field<N>;

    fn mul(self, other: i64) -> Self {
        Self {
            v: (self.v * other) % (self.n() as i64),
        }
    }
}
impl<N: Unsigned> ops::MulAssign for Field<N> {
    fn mul_assign(&mut self, other: Field<N>) {
        // assert_eq!(self.n, other.n);
        self.v = (self.v * other.v) % (self.n() as i64);
    }
}
impl<N: Unsigned> ops::MulAssign<i64> for Field<N> {
    fn mul_assign(&mut self, other: i64) {
        self.v = (self.v * other) % (self.n() as i64);
    }
}
impl<N: Unsigned> ops::Div for Field<N> {
    type Output = Field<N>;
    fn div(self, other: Field<N>) -> Self {
        // assert_eq!(self.n, other.n);
        self * (!other)
    }
}
impl<N: Unsigned> ops::DivAssign for Field<N> {
    fn div_assign(&mut self, other: Field<N>) {
        // assert_eq!(self.n, other.n);
        *self *= !other;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{U3};

    #[test]
    fn add_test() {
        let mut x = Field::<U3>::new(1) + Field::<U3>::new(4);
        assert_eq!(x, Field::<U3>::new(2));
        x += x.clone();
        assert_eq!(x, Field::<U3>::new(1));
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
