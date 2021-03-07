use std::ops::{Rem, RemAssign};

use crate::Polynomial;

impl Rem<isize> for Polynomial {
    type Output = Self;
    fn rem(mut self, rhs: isize) -> Self::Output {
        if self.is_zero() {
            self
        } else {
            for i in 0..=self.degree() {
                self.coeffs[i as usize] %= rhs;
            }
            self.reduce();
            self
        }
    }
}

impl<'a> Rem<isize> for &'a Polynomial {
    type Output = Polynomial;
    fn rem(self, rhs: isize) -> Self::Output {
        if self.is_zero() {
            Polynomial::zero()
        } else {
            let mut coeffs = self.coeffs.clone();
            for i in 0..=self.degree() {
                coeffs[i as usize] %= rhs;
            }
            let mut output = Polynomial { coeffs };
            output.reduce();
            output
        }
    }
}

impl RemAssign<isize> for Polynomial {
    fn rem_assign(&mut self, rhs: isize) {
        if !self.is_zero() {
            for i in 0..=self.degree() {
                self.coeffs[i as usize] %= rhs;
            }
            self.reduce();
        }
    }
}
