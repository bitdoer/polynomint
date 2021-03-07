use std::ops::{Mul, MulAssign};

use crate::Polynomial;

impl Mul for Polynomial {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let sdeg = self.degree();
        let rdeg = rhs.degree();
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let mut coeffs = Vec::new();
        for i in 0..=(sdeg + rdeg) {
            let mut acc = 0;
            for n in 0..=i {
                if n <= self.degree() && (i - n) <= rhs.degree() {
                    acc += self.coeffs[n as usize] * rhs.coeffs[(i - n) as usize];
                }
            }
            coeffs.push(acc);
        }
        let mut output = Self { coeffs };
        output.reduce();
        output
    }
}

impl<'a> Mul<&'a Polynomial> for &'a Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return Polynomial::zero();
        }
        let mut coeffs = Vec::new();
        for i in 0..=(self.degree() + rhs.degree()) {
            let mut acc = 0;
            for n in 0..=i {
                if n <= self.degree() && (i - n) <= rhs.degree() {
                    acc += self.coeffs[n as usize] * rhs.coeffs[(i - n) as usize];
                }
            }
            coeffs.push(acc);
        }
        let mut output = Polynomial::new(coeffs);
        output.reduce();
        output
    }
}

impl Mul<isize> for Polynomial {
    type Output = Self;
    fn mul(mut self, rhs: isize) -> Self::Output {
        if self.is_zero() || rhs == 0 {
            Polynomial::zero()
        } else {
            for i in 0..=self.degree() {
                self.coeffs[i as usize] *= rhs;
            }
            self
        }
    }
}

impl<'a> Mul<isize> for &'a Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: isize) -> Self::Output {
        if self.is_zero() || rhs == 0 {
            Polynomial::zero()
        } else {
            let mut coeffs = self.coeffs.clone();
            for i in 0..=self.degree() {
                coeffs[i as usize] *= rhs;
            }
            Polynomial { coeffs }
        }
    }
}

impl MulAssign for Polynomial {
    fn mul_assign(&mut self, rhs: Self) {
        if self.is_zero() || rhs.is_zero() {
            *self = Polynomial::zero();
        }
        let mut coeffs = Vec::new();
        for i in 0..=(self.degree() + rhs.degree()) {
            let mut acc = 0;
            for n in 0..=i {
                if n <= self.degree() && (i - n) <= rhs.degree() {
                    acc += self.coeffs[n as usize] * rhs.coeffs[(i - n) as usize];
                }
            }
            coeffs.push(acc);
        }
        *self = Polynomial { coeffs };
        self.reduce();
    }
}

impl<'a> MulAssign<&'a Polynomial> for Polynomial {
    fn mul_assign(&mut self, rhs: &Self) {
        if self.is_zero() || rhs.is_zero() {
            *self = Polynomial::zero();
        }
        let mut coeffs = Vec::new();
        for i in 0..=(self.degree() + rhs.degree()) {
            let mut acc = 0;
            for n in 0..=i {
                if n <= self.degree() && (i - n) <= rhs.degree() {
                    acc += self.coeffs[n as usize] * rhs.coeffs[(i - n) as usize];
                }
            }
            coeffs.push(acc);
        }
        *self = Polynomial { coeffs };
        self.reduce();
    }
}

impl MulAssign<isize> for Polynomial {
    fn mul_assign(&mut self, rhs: isize) {
        if self.is_zero() || rhs == 0 {
            *self = Polynomial::zero();
        } else {
            for i in 0..=self.degree() {
                self.coeffs[i as usize] *= rhs;
            }
        }
    }
}
