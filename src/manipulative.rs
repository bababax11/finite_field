use super::modulo::Field;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
#[derive(Clone, Debug, PartialEq)]
pub struct Manipulative<T>
where
    T: Copy,
{
    factors: Vec<T>,
}
impl<T> Manipulative<T>
where
    T: std::fmt::Debug
        + Copy
        + AddAssign
        + SubAssign
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + Eq,
{
    pub fn new(factors: Vec<T>) -> Self {
        if factors.len() == 0 {
            panic!("係数が空");
        }
        Self { factors: factors }
    }
    pub fn divide_by(&self, other: &Manipulative<T>) -> (Manipulative<T>, Manipulative<T>) {
        let mut man_r = self.clone();
        let mut q = Vec::with_capacity(self.factors.len());
        q.resize_with(self.factors.len(), Default::default);
        let (j, d) = (|| { // otherの先頭要素を返す
            for (j, d) in other.factors.iter().enumerate().rev() {
                if *d != Default::default() {
                    return Ok((j, *d))
                }
            }
            Err("devide by zero")
        })().unwrap();
        'outer: loop {
            let mut it = man_r.factors.iter().enumerate().rev();
            let mut l;
            let mut i;
            while {
                let ll = it.next();
                match ll {
                    Some((i_, l_)) => {
                        l = *l_;
                        i = i_;
                        if i < j {
                            break 'outer;
                        }
                        l == Default::default()
                    }
                    None => break 'outer,
                }
            } {}
            let a = l / d;
            unsafe {
                *q.get_unchecked_mut(i - j) = a;
            }
            let mut o = other.clone();
            let mut v = vec![Default::default(); i - j];
            v.push(a);
            o *= Manipulative::new(v);
            man_r -= o;
        
        }
        (Manipulative::new(q), man_r)
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
            self.factors.reserve(max_deg);
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
            self.factors.reserve(max_deg);
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
impl MulAssign for Manipulative<Field> {
    fn mul_assign(&mut self, other: Manipulative<Field>) {
        let deg = self.factors.len() + other.factors.len() - 1;
        let mut new_factors = Vec::with_capacity(deg);
        new_factors.resize(
            deg,
            Field::new(0, unsafe { self.factors.get_unchecked(0).n }),
        );
        for (i, l) in self.factors.iter().enumerate() {
            for (j, r) in other.factors.iter().enumerate() {
                unsafe {
                    *new_factors.get_unchecked_mut(i + j) += *l * *r;
                }
            }
        }
        self.factors = new_factors;
    }
}
impl<T> MulAssign for Manipulative<T>
where
    T: Copy + Default + AddAssign + Mul<Output = T>,
{
    fn mul_assign(&mut self, other: Manipulative<T>) {
        let deg = self.factors.len() + other.factors.len() - 1; // -1-1+1
        let mut new_factors = Vec::with_capacity(deg);
        new_factors.resize_with(deg, Default::default);
        for (i, l) in self.factors.iter().enumerate() {
            for (j, r) in other.factors.iter().enumerate() {
                unsafe {
                    *new_factors.get_unchecked_mut(i + j) += *l * *r;
                }
            }
        }
        self.factors = new_factors;
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
impl<T> Mul for Manipulative<T>
where
    T: Copy + Default + AddAssign + Mul<Output = T>,
{
    type Output = Manipulative<T>;

    fn mul(self, other: Manipulative<T>) -> Self {
        let mut result = self.clone();
        result *= other;
        result
    }
}
impl Mul for Manipulative<Field> {
    type Output = Manipulative<Field>;

    fn mul(self, other: Manipulative<Field>) -> Self {
        let mut result = self.clone();
        result *= other;
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
        assert_eq!((a.clone() + b.clone()).factors, [3, 7, 3]);
        assert_eq!((b + a).factors, [3, 7, 3]);
    }
    #[test]
    fn sub_test() {
        let a = Manipulative::new(vec![1, 5]);
        let b = Manipulative::new(vec![2, 2, 3]);
        assert_eq!((a.clone() - b.clone()).factors, [-1, 3, -3]);
        assert_eq!((b - a).factors, [1, -3, 3]);
    }
    #[test]
    fn mul_test() {
        let a = Manipulative::new(vec![1, 2]);
        let b = Manipulative::new(vec![1, 1]);
        assert_eq!((a.clone() * b.clone()).factors, [1, 3, 2]);
        assert_eq!((b * a).factors, [1, 3, 2]);
    }
    #[test]
    fn div_test() {
        let a = Manipulative::new(vec![1, 2, 1]);
        let b = Manipulative::new(vec![1, 1]);
        assert_eq!(
            a.divide_by(&b),
            (
                Manipulative::new(vec![1, 1, 0]),
                Manipulative::new(vec![0, 0, 0])
            )
        );
        let a = Manipulative::new(vec![-2, -3, -1, 0]);
        let b = Manipulative::new(vec![1, 1, 0, 0, 0]);
        assert_eq!(
            a.divide_by(&b),
            (
                Manipulative::new(vec![-2, -1, 0, 0]),
                Manipulative::new(vec![0, 0, 0, 0, 0, 0])
            )
        );
        let a = Manipulative::new(vec![2, 3, 1, 0]);
        let b = Manipulative::new(vec![1, 0, 0]);
        assert_eq!(
            a.divide_by(&b),
            (
                Manipulative::new(vec![2, 3, 1, 0]),
                Manipulative::new(vec![0, 0, 0, 0, 0])
            )
        );
        let a = Manipulative::new(vec![2, 3, 1, 0]);
        let b = Manipulative::new(vec![1, 1, 1, 0]);
        assert_eq!(
            a.divide_by(&b),
            (
                Manipulative::new(vec![1, 0, 0, 0]),
                Manipulative::new(vec![1, 2, 0, 0])
            )
        );
    }
}
