use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
#[derive(Clone, Debug, PartialEq)]
struct Manipulative<T>
where
    T: Copy,
{
    factors: Vec<T>,
}
impl<T> Manipulative<T>
where
    T: Copy + AddAssign + SubAssign + Neg<Output = T> + MulAssign + DivAssign,
{
    pub fn new(factors: Vec<T>) -> Self {
        Self { factors: factors }
    }
}
impl<T> AddAssign for Manipulative<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, other: Manipulative<T>) {
        let min_deg;
        let max_deg;
        let is_self_shorter;
        if self.factors.len() < other.factors.len() {
            min_deg = self.factors.len();
            max_deg = other.factors.len();
            is_self_shorter = true;
        } else {
            min_deg = other.factors.len();
            max_deg = self.factors.len();
            is_self_shorter = false;
        }
        for (l, r) in self.factors.iter_mut().zip(other.factors.iter()) {
            *l += *r;
        }
        if is_self_shorter {
            for i in min_deg..max_deg {
                self.factors
                    .push(unsafe { *other.factors.get_unchecked(i) })
            }
        }
    }
}
impl<T> SubAssign for Manipulative<T>
where
    T: Copy + SubAssign + Neg<Output = T>,
{
    fn sub_assign(&mut self, other: Manipulative<T>) {
        let min_deg;
        let max_deg;
        let is_self_shorter;
        if self.factors.len() < other.factors.len() {
            min_deg = self.factors.len();
            max_deg = other.factors.len();
            is_self_shorter = true;
        } else {
            min_deg = other.factors.len();
            max_deg = self.factors.len();
            is_self_shorter = false;
        }
        for (l, r) in self.factors.iter_mut().zip(other.factors.iter()) {
            *l -= *r;
        }
        if is_self_shorter {
            for i in min_deg..max_deg {
                self.factors
                    .push(-unsafe { *other.factors.get_unchecked(i) })
            }
        }
    }
}
impl<T> Add for Manipulative<T>
where
    T: Copy + AddAssign,
{
    type Output = Manipulative<T>;

    fn add(self, other: Manipulative<T>) -> Self {
        let mut result = self.clone();
        result += other;
        result
    }
}
impl<T> Sub for Manipulative<T>
where
    T: Copy + SubAssign + Neg<Output = T>,
{
    type Output = Manipulative<T>;

    fn sub(self, other: Manipulative<T>) -> Self {
        let mut result = self.clone();
        result -= other;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let a = Manipulative::new(vec![1, 5]);
        let b = Manipulative::new(vec![2, 2, 3]);
        assert_eq!((a.clone() + b.clone()).factors, vec![3, 7, 3]);
        assert_eq!((b + a).factors, vec![3, 7, 3]);
    }
    #[test]
    fn sub_test() {
        let a = Manipulative::new(vec![1, 5]);
        let b = Manipulative::new(vec![2, 2, 3]);
        assert_eq!((a.clone() - b.clone()).factors, vec![-1, 3, -3]);
        assert_eq!((b - a).factors, vec![1, -3, 3]);
    }
}
