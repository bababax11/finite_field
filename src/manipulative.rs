use std::ops::{Add, Div, Mul, Neg, Sub};
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
impl <T> Manipulative<T> 
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    pub fn new(factors: Vec<T>) -> Self {
        Self {
            factors: factors
        }
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
        let shorter;
        let longer;
        if self.factors.len() < other.factors.len() {
            shorter = self.factors;
            longer = other.factors;
        } else {
            shorter = other.factors;
            longer = self.factors;
        }
        let mut new_factors = Vec::with_capacity(longer.len());
        for (s, l) in shorter.iter().zip(longer.iter()) {
            new_factors.push(s.clone() + l.clone());
        }
        for i in shorter.len()..longer.len() {
            new_factors.push(unsafe { *longer.get_unchecked(i) })
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
        assert_eq!((a + b).factors, vec![3, 7, 3]);
        
    }
}