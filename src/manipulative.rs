use super::modulo::Field;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
#[derive(Clone, Debug, PartialEq)]
pub struct Manipulative<T> {
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
        + PartialEq,
{
    pub fn new(factors: Vec<T>) -> Self {
        if factors.len() == 0 {
            panic!("係数が空");
        }
        Self { factors: factors }
    }
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
        + MulAssign
        + Div<Output = T>
        + Default
        + PartialEq,
{
    pub fn divide_by(
        &self,
        other: &Manipulative<T>,
    ) -> Result<(Manipulative<T>, Manipulative<T>), &'static str> {
        let mut man_r = self.clone();
        let mut q = Vec::with_capacity(self.factors.len());
        q.resize_with(self.factors.len(), Default::default);
        let (j, d) = (|| {
            // otherの先頭要素を返す
            for (j, d) in other.factors.iter().enumerate().rev() {
                if *d != Default::default() {
                    return Ok((j, *d));
                }
            }
            Err("devided by zero")
        })()?;
        'outer: loop {
            let mut it = man_r.factors.iter().enumerate().rev();
            let mut l;
            let mut i;
            while {
                let ll = it.next();
                match ll {
                    Some((i_, l_)) => {
                        if i_ < j {
                            break 'outer;
                        }
                        l = *l_;
                        i = i_;
                        l == Default::default() // 0だったら繰り返す
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
            o *= &Manipulative::new(v);
            man_r -= &o;
        }
        Ok((Manipulative::new(q), man_r))
    }
    pub fn assign(&self, x: T) -> T {
        let mut result = Default::default();
        for a in self.factors.iter().rev() {
            result *= x;
            result += *a;
        }
        result
    }
}
impl Manipulative<Field> {
    pub fn divide_by(
        &self,
        other: &Manipulative<Field>,
    ) -> Result<(Manipulative<Field>, Manipulative<Field>), &'static str> {
        let mut man_r = self.clone();
        let mut q = Vec::with_capacity(self.factors.len());
        let default = Field::new(0, unsafe { self.factors.get_unchecked(0).n });
        q.resize(self.factors.len(), default);
        let (j, d) = (|| {
            // otherの先頭要素を返す
            for (j, d) in other.factors.iter().enumerate().rev() {
                if *d != default {
                    return Ok((j, *d));
                }
            }
            Err("devide by zero")
        })()?;
        'outer: loop {
            let mut it = man_r.factors.iter().enumerate().rev();
            let mut l;
            let mut i;
            while {
                let ll = it.next();
                match ll {
                    Some((i_, l_)) => {
                        if i_ < j {
                            break 'outer;
                        }
                        l = *l_;
                        i = i_;
                        l == default // 0だったら繰り返す
                    }
                    None => break 'outer,
                }
            } {}
            let a = l / d;
            unsafe {
                *q.get_unchecked_mut(i - j) = a;
            }
            let mut o = other.clone();
            let mut v = vec![default; i - j];
            v.push(a);
            o *= &Manipulative::new(v);
            man_r -= &o;
        }
        Ok((Manipulative::new(q), man_r))
    }
    pub fn assign(&self, x: Field) -> Field {
        let mut result = Field::new(0, x.n);
        for a in self.factors.iter().rev() {
            result *= x;
            result += *a;
        }
        result
    }
    pub fn deg(&self) -> i64 {
        let default = Field::new(0, unsafe { self.factors.get_unchecked(0).n });
        for (i, d) in self.factors.iter().enumerate().rev() {
            if *d != default {
                return i as i64;
            }
        }
        -1 // -inftyでも-1とする
    }
    pub fn diff(&self) -> Manipulative<Field> {
        if self.factors.len() == 1 {
            return Manipulative::new(vec![Field::new(0, unsafe {
                self.factors.get_unchecked(0).n
            })]);
        }
        let mut new_factors = Vec::with_capacity(self.factors.len() - 1);
        for (i, a) in self.factors.iter().enumerate() {
            if i != 0 {
                new_factors.push(*a * i as i64);
            }
        }
        Manipulative::new(new_factors)
    }
}
impl<T> AddAssign<&Manipulative<T>> for Manipulative<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, other: &Manipulative<T>) {
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
impl<T> SubAssign<&Manipulative<T>> for Manipulative<T>
where
    T: Copy + SubAssign + Neg<Output = T>,
{
    fn sub_assign(&mut self, other: &Manipulative<T>) {
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
impl Mul<&Manipulative<Field>> for &Manipulative<Field> {
    type Output = Manipulative<Field>;
    fn mul(self, other: &Manipulative<Field>) -> Manipulative<Field> {
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
        Manipulative::new(new_factors)
    }
}
impl<T> Mul<&Manipulative<T>> for &Manipulative<T>
where
    T: std::fmt::Debug
        + Copy
        + AddAssign
        + SubAssign
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + Default
        + PartialEq,
{
    type Output = Manipulative<T>;
    fn mul(self, other: &Manipulative<T>) -> Manipulative<T> {
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
        Manipulative::new(new_factors)
    }
}
impl<T> Add<&Manipulative<T>> for Manipulative<T>
where
    T: Copy + AddAssign,
{
    type Output = Manipulative<T>;

    fn add(mut self, other: &Manipulative<T>) -> Self {
        self += other;
        self
    }
}
impl<T> Sub<&Manipulative<T>> for Manipulative<T>
where
    T: Copy + SubAssign + Neg<Output = T>,
{
    type Output = Manipulative<T>;

    fn sub(mut self, other: &Manipulative<T>) -> Self {
        self -= other;
        self
    }
}
impl<T> MulAssign<&Manipulative<T>> for Manipulative<T>
where
    T: std::fmt::Debug
        + Copy
        + AddAssign
        + SubAssign
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + Default
        + PartialEq,
{
    fn mul_assign(&mut self, other: &Manipulative<T>) {
        let new_factors = (&(*self) * other).factors;
        self.factors = new_factors;
    }
}
impl MulAssign<&Manipulative<Field>> for Manipulative<Field> {
    fn mul_assign(&mut self, other: &Manipulative<Field>) {
        let new_factors = (&(*self) * other).factors;
        self.factors = new_factors;
    }
}
impl<T: PartialEq + fmt::Display + Default> fmt::Display for Manipulative<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, l) in self.factors.iter().enumerate().rev() {
            if *l != Default::default() {
                if i > 1 {
                    let _ = write!(f, "{} x^{} + ", l, i);
                } else if i == 1 {
                    let _ = write!(f, "{} x + ", l);
                } else {
                    let _ = write!(f, "{}", l);
                }
            } else if i == 0 {
                let _ = write!(f, "{}", l);
            }
        }
        write!(f, "")
    }
}
impl fmt::Display for Manipulative<Field> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let default = Field::new(0, unsafe { self.factors.get_unchecked(0).n });
        for (i, l) in self.factors.iter().enumerate().rev() {
            if *l != default {
                if i > 1 {
                    let _ = write!(f, "{} x^{} + ", (*l).v, i);
                } else if i == 1 {
                    let _ = write!(f, "{} x + ", (*l).v);
                } else {
                    let _ = write!(f, "{}", (*l).v);
                }
            } else if i == 0 {
                let _ = write!(f, "{}", (*l).v);
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let a = Manipulative::new(vec![1, 5]);
        let b = Manipulative::new(vec![2, 2, 3]);
        assert_eq!((a.clone() + &b).factors, [3, 7, 3]);
        assert_eq!((b + &a).factors, [3, 7, 3]);
    }
    #[test]
    fn sub_test() {
        let a = Manipulative::new(vec![1, 5]);
        let b = Manipulative::new(vec![2, 2, 3]);
        assert_eq!((a.clone() - &b).factors, [-1, 3, -3]);
        assert_eq!((b - &a).factors, [1, -3, 3]);
    }
    #[test]
    fn mul_test() {
        let a = Manipulative::new(vec![1, 2]);
        let b = Manipulative::new(vec![1, 1]);
        assert_eq!((&a * &b).factors, [1, 3, 2]);
        assert_eq!((&b * &a).factors, [1, 3, 2]);
    }
    #[test]
    fn div_test() {
        let a = Manipulative::new(vec![1, 2, 1]);
        let b = Manipulative::new(vec![1, 1]);
        assert_eq!(
            a.divide_by(&b).unwrap(),
            (
                Manipulative::new(vec![1, 1, 0]),
                Manipulative::new(vec![0, 0, 0])
            )
        );
        let a = Manipulative::new(vec![-2, -3, -1, 0]);
        let b = Manipulative::new(vec![1, 1, 0, 0, 0]);
        assert_eq!(
            a.divide_by(&b).unwrap(),
            (
                Manipulative::new(vec![-2, -1, 0, 0]),
                Manipulative::new(vec![0, 0, 0, 0, 0, 0])
            )
        );
        let a = Manipulative::new(vec![2, 3, 1, 0]);
        let b = Manipulative::new(vec![1, 0, 0]);
        assert_eq!(
            a.divide_by(&b).unwrap(),
            (
                Manipulative::new(vec![2, 3, 1, 0]),
                Manipulative::new(vec![0, 0, 0, 0, 0])
            )
        );
        let a = Manipulative::new(vec![2, 3, 1, 0]);
        let b = Manipulative::new(vec![0, 1, 0]);
        assert_eq!(
            a.divide_by(&b).unwrap(),
            (
                Manipulative::new(vec![3, 1, 0, 0]),
                Manipulative::new(vec![2, 0, 0, 0])
            )
        );
        let a = Manipulative::new(vec![2, 3, 1, 0]);
        let b = Manipulative::new(vec![1, 1, 1, 0]);
        assert_eq!(
            a.divide_by(&b).unwrap(),
            (
                Manipulative::new(vec![1, 0, 0, 0]),
                Manipulative::new(vec![1, 2, 0, 0])
            )
        );
    }
    #[test]
    fn assign_test() {
        let a = Manipulative::new(vec![2, -3, 1, 0]);
        assert_eq!(a.assign(3), 2);
    }
}
