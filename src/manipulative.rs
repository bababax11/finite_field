use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Clone, Debug, PartialEq)]
struct Manipulative<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    factors: Vec<T>,
}
impl<T> Manipulative<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    pub fn new(factors: Vec<T>) -> Self {
        Self { factors: factors }
    }
}
impl<T> Add for Manipulative<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    type Output = Manipulative<T>;

    fn add(self, other: Manipulative<T>) -> Self {
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
        let mut new_factors = Vec::with_capacity(max_deg);
        for (l, r) in self.factors.iter().zip(other.factors.iter()) {
            new_factors.push(l.clone() + r.clone());
        }
        if is_self_shorter {
            for i in min_deg..max_deg {
                new_factors.push(unsafe { *other.factors.get_unchecked(i) })
            }
        } else {
            for i in min_deg..max_deg {
                new_factors.push(unsafe { *self.factors.get_unchecked(i) })
            }
        }
        Self {
            factors: new_factors,
        }
    }
}
impl<T> Sub for Manipulative<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    type Output = Manipulative<T>;

    fn sub(self, other: Manipulative<T>) -> Self {
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
        let mut new_factors = Vec::with_capacity(max_deg);
        for (l, r) in self.factors.iter().zip(other.factors.iter()) {
            new_factors.push(l.clone() - r.clone());
        }
        if is_self_shorter {
            for i in min_deg..max_deg {
                new_factors.push(- unsafe { *other.factors.get_unchecked(i) })
            }
        } else {
            for i in min_deg..max_deg {
                new_factors.push(unsafe { *self.factors.get_unchecked(i) })
            }
        }
        Self {
            factors: new_factors,
        }
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
