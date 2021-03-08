use std::ops::{Neg, Sub, SubAssign};

use crate::Polynomial;

impl Sub for Polynomial {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            rhs
        } else if rhs.is_zero() {
            self
        } else {
            for i in 0..=(std::cmp::max(self.degree(), rhs.degree())) {
                if i > self.degree() {
                    // if i > deg self, then by the loop condition, i <= deg rhs,
                    // so the indexing is valid
                    self.coeffs.push(-rhs.coeffs[i as usize]);
                } else if i > rhs.degree() {
                    // if i > deg rhs, then whatever higher coefficients remain in self
                    // are already correct
                    break;
                } else {
                    // otherwise just sub from self---this makes the subtraction be in-place
                    self.coeffs[i as usize] -= rhs.coeffs[i as usize];
                }
            }
            self.reduce();
            self
        }
    }
}

impl<'a> Sub<&'a Polynomial> for &'a Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut coeffs = Vec::new();
        for i in 0..=(std::cmp::max(self.degree(), rhs.degree())) {
            if i > self.degree() {
                coeffs.push(-rhs.coeffs[i as usize]);
            } else if i > rhs.degree() {
                coeffs.push(self.coeffs[i as usize]);
            } else {
                coeffs.push(self.coeffs[i as usize] - rhs.coeffs[i as usize]);
            }
        }
        let mut output = Polynomial::new(coeffs);
        output.reduce();
        output
    }
}

impl Sub<isize> for Polynomial {
    type Output = Self;
    fn sub(mut self, rhs: isize) -> Self::Output {
        if self.is_zero() {
            Polynomial::constant(-rhs)
        } else if rhs == 0 {
            self
        } else {
            self.coeffs[0] -= rhs;
            self.reduce();
            self
        }
    }
}

impl<'a> Sub<isize> for &'a Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: isize) -> Self::Output {
        if self.is_zero() {
            Polynomial::constant(-rhs)
        } else if rhs == 0 {
            self.clone()
        } else {
            let mut coeffs = self.coeffs.clone();
            coeffs[0] -= rhs;
            let mut output = Polynomial { coeffs };
            output.reduce();
            output
        }
    }
}

impl Neg for Polynomial {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for i in self.coeffs.iter_mut() {
            *i = -*i;
        }
        self
    }
}

impl Neg for &Polynomial {
    type Output = Polynomial;

    fn neg(self) -> Self::Output {
        let mut coeffs = self.coeffs.clone();
        for i in coeffs.iter_mut() {
            *i = -*i;
        }
        let mut output = Polynomial { coeffs };
        output.reduce();
        output
    }
}

impl SubAssign for Polynomial {
    fn sub_assign(&mut self, rhs: Self) {
        if self.is_zero() {
            *self = -rhs;
        } else if !rhs.is_zero() {
            for i in 0..=(std::cmp::max(self.degree(), rhs.degree())) {
                if i > self.degree() {
                    // if i > deg self, then by the loop condition, i <= deg rhs,
                    // so the indexing is valid
                    self.coeffs.push(-rhs.coeffs[i as usize]);
                } else if i > rhs.degree() {
                    // if i > deg rhs, then whatever higher coefficients remain in self
                    // are already correct
                    break;
                } else {
                    // otherwise just sub from self---this makes the subtraction be in-place
                    self.coeffs[i as usize] -= rhs.coeffs[i as usize];
                }
            }
            self.reduce();
        }
    }
}

impl<'a> SubAssign<&'a Polynomial> for Polynomial {
    fn sub_assign(&mut self, rhs: &Self) {
        if self.is_zero() {
            *self = -rhs;
        } else if !rhs.is_zero() {
            for i in 0..=(std::cmp::max(self.degree(), rhs.degree())) {
                if i > self.degree() {
                    // if i > deg self, then by the loop condition, i <= deg rhs,
                    // so the indexing is valid
                    self.coeffs.push(-rhs.coeffs[i as usize]);
                } else if i > rhs.degree() {
                    // if i > deg rhs, then whatever higher coefficients remain in self
                    // are already correct
                    break;
                } else {
                    // otherwise just sub from self---this makes the subtraction be in-place
                    self.coeffs[i as usize] -= rhs.coeffs[i as usize];
                }
            }
            self.reduce();
        }
    }
}

impl SubAssign<isize> for Polynomial {
    fn sub_assign(&mut self, rhs: isize) {
        if self.is_zero() {
            self.coeffs.push(-rhs);
        } else if rhs != 0 {
            self.coeffs[0] -= rhs;
            self.reduce();
        }
    }
}
