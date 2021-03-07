use std::ops::{Add, AddAssign};

use crate::Polynomial;

impl Add for Polynomial {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            rhs
        } else if rhs.is_zero() {
            self
        } else {
            for i in 0..=(std::cmp::max(self.degree(), rhs.degree())) {
                if i > self.degree() {
                    // if i > deg self, then by the loop condition, i <= deg rhs,
                    // so the indexing is valid
                    self.coeffs.push(rhs.coeffs[i as usize]);
                } else if i > rhs.degree() {
                    // if i > deg rhs, then whatever higher coefficients remain in self
                    // are already correct
                    break;
                } else {
                    // otherwise just add to self---this makes the addition be in-place
                    self.coeffs[i as usize] += rhs.coeffs[i as usize];
                }
            }
            self.reduce();
            self
        }
    }
}

impl<'a> Add<&'a Polynomial> for &'a Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: Self) -> Self::Output {
        let mut coeffs = Vec::new();
        for i in 0..=(std::cmp::max(self.degree(), rhs.degree())) {
            if i > self.degree() {
                coeffs.push(rhs.coeffs[i as usize]);
            } else if i > rhs.degree() {
                coeffs.push(self.coeffs[i as usize]);
            } else {
                coeffs.push(self.coeffs[i as usize] + rhs.coeffs[i as usize]);
            }
        }
        let mut output = Polynomial::new(coeffs);
        output.reduce();
        output
    }
}

impl Add<isize> for Polynomial {
    type Output = Self;
    fn add(mut self, rhs: isize) -> Self::Output {
        if self.is_zero() {
            Polynomial::constant(rhs)
        } else if rhs == 0 {
            self
        } else {
            self.coeffs[0] += rhs;
            self.reduce();
            self
        }
    }
}

impl<'a> Add<isize> for &'a Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: isize) -> Self::Output {
        if self.is_zero() {
            Polynomial::constant(rhs)
        } else if rhs == 0 {
            self.clone()
        } else {
            let mut coeffs = self.coeffs.clone();
            coeffs[0] += rhs;
            let mut output = Polynomial { coeffs };
            output.reduce();
            output
        }
    }
}

impl AddAssign for Polynomial {
    fn add_assign(&mut self, rhs: Self) {
        if self.is_zero() {
            self.coeffs = rhs.coeffs;
        } else if !rhs.is_zero() {
            for i in 0..=(std::cmp::max(self.degree(), rhs.degree())) {
                if i > self.degree() {
                    // if i > deg self, then by the loop condition, i <= deg rhs,
                    // so the indexing is valid
                    self.coeffs.push(rhs.coeffs[i as usize]);
                } else if i > rhs.degree() {
                    // if i > deg rhs, then whatever higher coefficients remain in self
                    // are already correct
                    break;
                } else {
                    // otherwise just add to self---this makes the addition be in-place
                    self.coeffs[i as usize] += rhs.coeffs[i as usize];
                }
            }
            self.reduce();
        }
    }
}

impl<'a> AddAssign<&'a Polynomial> for Polynomial {
    fn add_assign(&mut self, rhs: &Self) {
        if self.is_zero() {
            self.coeffs = rhs.coeffs.clone();
        } else if !rhs.is_zero() {
            for i in 0..=(std::cmp::max(self.degree(), rhs.degree())) {
                if i > self.degree() {
                    // if i > deg self, then by the loop condition, i <= deg rhs,
                    // so the indexing is valid
                    self.coeffs.push(rhs.coeffs[i as usize]);
                } else if i > rhs.degree() {
                    // if i > deg rhs, then whatever higher coefficients remain in self
                    // are already correct
                    break;
                } else {
                    // otherwise just add to self---this makes the addition be in-place
                    self.coeffs[i as usize] += rhs.coeffs[i as usize];
                }
            }
            self.reduce();
        }
    }
}

impl AddAssign<isize> for Polynomial {
    fn add_assign(&mut self, rhs: isize) {
        if self.is_zero() {
            self.coeffs.push(rhs);
        } else if rhs != 0 {
            self.coeffs[0] += rhs;
            self.reduce();
        }
    }
}
