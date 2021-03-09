use crate::Polynomial;

impl Polynomial {
    /// Gives a new polynomial equal to the old one times x.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let first = poly![1, 2, 3];
    /// let second = first.times_x();
    ///
    /// assert_eq!(second, poly![0, 1, 2, 3]);
    /// ```
    pub fn times_x(&self) -> Self {
        let mut coeffs = vec![0];
        coeffs.append(&mut self.coeffs.clone());
        Self { coeffs }
    }

    /// Gives a new polynomial equal to the remainder of the old one when taken
    /// modulo `n`.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let poly = poly![6, -5, 3, -7, 4];
    /// assert_eq!(poly.rem_euclid(2), poly![0, 1, 1, 1]);
    /// assert_eq!(poly.rem_euclid(4), poly![2, 3, 3, 1]);
    /// assert_eq!(poly.rem_euclid(5), poly![1, 0, 3, 3, 4]);
    /// ```
    pub fn rem_euclid(&self, n: isize) -> Self {
        if self.is_zero() {
            Polynomial::zero()
        } else {
            let mut coeffs = self.coeffs.clone();
            for i in 0..=self.degree() {
                coeffs[i as usize] = coeffs[i as usize].rem_euclid(n);
            }
            let mut output = Polynomial { coeffs };
            output.reduce();
            output
        }
    }

    /// Creates a new polynomial which is the derivative of the old one.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let poly1 = poly![1, -2, 5, 4]; // 4x^3 + 5x^2 - 2x + 1
    /// assert_eq!(poly1.derivative(), poly![-2, 10, 12]); // deriv. is 12x^2 + 10x - 2
    /// let poly2 = poly![192, 3, -4, -9, 0, 38]; // 38x^5 - 9x^3 - 4x^2 + 3x + 192
    /// assert_eq!(poly2.derivative(), poly![3, -8, -27, 0, 190]); // deriv. is 190x^4 - 27x^2 - 8x + 3
    /// ```
    pub fn derivative(&self) -> Self {
        if self.degree() <= 0 {
            Self::zero()
        } else {
            let mut coeffs = Vec::new();
            for i in 0..self.degree() {
                coeffs.push((i + 1) * self.coeffs[i as usize + 1]);
            }
            let mut output = Self { coeffs };
            output.reduce();
            output
        }
    }

    /// Plugs in a specific `isize` value `x` to the polynomial.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{poly, Polynomial};
    ///
    /// let poly1 = poly![5,2,1];
    /// let poly2 = poly![-5,4,-3,-1];
    ///
    /// assert_eq!(poly1.eval(1), 8);
    /// assert_eq!(poly2.eval(1), -5);
    ///
    /// assert_eq!(poly1.eval(-2), 5);
    /// assert_eq!(poly2.eval(-2), -17);
    /// ```
    pub fn eval(&self, x: isize) -> isize {
        let mut acc = 0;
        // take a polynomial like 5x^2 + 2x + 3: we can get this by: 0 *= x -> 0
        //                                                             += 5 -> 5
        //                                                             *= x -> 5x
        //                                                             += 2 -> 5x + 2
        //                                                             *= x -> 5x^2 + 2x
        //                                                             += 3 -> 5x^2 + 2x + 3
        // this motivates the loop
        for &i in self.coeffs.iter().rev() {
            acc *= x;
            acc += i;
        }
        acc
    }

    /// Returns `true` if `x` is a root of the polynomial; otherwise returns `false`.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    /// let poly = poly![-2, 1] * poly![-4, 1] * poly![3, 1];
    ///
    /// assert_eq!(poly, poly![24, -10, -3, 1]);
    /// assert!(poly.has_root(2));
    /// assert!(poly.has_root(4));
    /// assert!(poly.has_root(-3));
    /// assert!(!poly.has_root(1));
    /// ```
    pub fn has_root(&self, x: isize) -> bool {
        self.eval(x) == 0
    }

    /// Returns `true` if `x` is a root of the polynomial taken modulo `div`; otherwise returns false.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let poly = poly![-2, 1] * poly![-6, 1];
    ///
    /// assert_eq!(poly, poly![12, -8, 1]);
    /// assert!(poly.has_root_mod(2, 5));
    /// assert!(poly.has_root_mod(1, 5));
    /// assert!(poly.has_root_mod(2, 3));
    /// assert!(poly.has_root_mod(0, 3));
    /// assert!(!poly.has_root_mod(4, 5));
    /// ```
    pub fn has_root_mod(&self, x: isize, div: isize) -> bool {
        self.eval(x).rem_euclid(div) == 0
    }

    /// If `a` is a root of `self`, returns `Some(p)` where `self = p * (x - a)`.
    /// (That is, if `a` is a root of `self`, this returns the result of factoring
    /// `x - a` out of `self`.) Otherwise returns `None`.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let poly = poly![12, -8, 1]; // x^2 - 8x + 12 = (x - 2)(x - 6)
    /// assert_eq!(poly.factor_root(2), Some(poly![-6, 1]));
    /// assert_eq!(poly.factor_root(6), Some(poly![-2, 1]));
    /// assert_eq!(poly.factor_root(5), None);
    /// ```
    pub fn factor_root(&self, a: isize) -> Option<Self> {
        // if not a root, we're done
        if !self.has_root(a) {
            None
        // if polynomial is zero, everything's a root, and the factoring gives zero again
        } else if self.is_zero() {
            Some(Self::zero())
        // if zero is a root, then we can just skip the constant and be done
        } else if a == 0 {
            Some(Self {
                coeffs: self.iter().skip(1).copied().collect(),
            })
        // otherwise, we know that the last coefficient b[0] of the output
        // will be -c[0]/a where c is self's coeff vec, and b[n] = (b[n-1] - c[n])/a
        // in general; thus the loop below does what we want---
        } else {
            let mut coeffs = Vec::new();
            // keep an accumulator,
            let mut acc = 0;
            for &coeff in self.iter().take(self.degree() as usize) {
                // and at each step, subtract c[n] and divide by a
                acc -= coeff;
                acc /= a;
                coeffs.push(acc);
            }
            Some(Self { coeffs })
        }
    }

    /// If `a` is a root of `self` and if `p` is a prime, this returns the
    /// result of factoring `x - a` out of `self`, if everything is considered
    /// a polynomial with coefficients modulo `p`. Otherwise returns `None`.
    ///
    /// The API demands that `p` be prime because factoring gets more complicated
    /// when the modulus is composite, like the integers mod 4---the example below,
    /// `x^2 - 8x + 12`, just becomes `x^2`, but `x^2 = x^2 + 4x + 4 = (x + 2)^2`,
    /// and unique factorization is lost.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let poly = poly![12, -8, 1]; // x^2 - 8x + 12 = (x - 2)(x - 6)
    ///                              // = x^2 + x (mod 3) = x(x - 2) or x(x + 1) mod 3
    ///                              // = x^2 + 2x + 2 = (x - 2)(x + 4) or (x - 1)(x + 3) mod 5
    /// assert_eq!(poly.factor_root_mod(2, 3), Some(poly![0, 1]));
    /// assert_eq!(poly.factor_root_mod(0, 3), Some(poly![1, 1]));
    /// assert_eq!(poly.factor_root_mod(1, 3), None);
    /// assert_eq!(poly.factor_root_mod(2, 5), Some(poly![4, 1]));
    /// assert_eq!(poly.factor_root_mod(1, 5), Some(poly![3, 1]));
    /// assert_eq!(poly.factor_root_mod(0, 5), None);
    ///
    /// let poly2 = poly![1, 0, 1]; // x^2 + 1 = (x + 1)^2 mod 2
    ///
    /// assert_eq!(poly2.factor_root_mod(1, 2), Some(poly![1, 1]));
    /// assert_eq!(poly2.factor_root_mod(0, 2), None);
    /// ```
    pub fn factor_root_mod(&self, a: isize, p: isize) -> Option<Self> {
        // if not a root or p isn't prime, we're done
        if !self.has_root_mod(a, p) || !Self::is_prime(p as usize) {
            None
        // if polynomial is zero, everything's a root, and the factoring gives zero again
        } else if self.is_zero() {
            Some(Self::zero())
        // if zero is a root, then we can just skip the constant, reduce mod div, and be done
        } else if a == 0 {
            let mut output = Self {
                coeffs: self.rem_euclid(p).iter().skip(1).copied().collect(),
            };
            output.reduce();
            Some(output)
        // otherwise, we know that the last coefficient b[0] of the output
        // will be -c[0]/a where c is self's coeff vec, and b[n] = (b[n-1] - c[n])/a
        // in general; thus the loop below does what we want---
        } else {
            let mut coeffs = Vec::new();
            // keep an accumulator,
            let mut acc = 0;
            for &coeff in self.rem_euclid(p).iter().take(self.degree() as usize) {
                // and at each step, subtract c[n] and divide by a
                acc -= coeff;
                acc *= Self::inv_mod_p(a.rem_euclid(p), p);
                coeffs.push(acc.rem_euclid(p));
            }
            Some(Self { coeffs })
        }
    }

    fn is_prime(p: usize) -> bool {
        if p == 2 || p == 3 {
            true
        } else if p == 1 || p % 2 == 0 || p % 3 == 0 {
            false
        } else {
            // we need only search for prime factors up to the sqrt of n;
            // every prime past 3 is either 1 or 5 mod 6, so we can quickly
            // reduce our search space to a size of approx sqrt(n)/3
            for i in (5..((p as f64).sqrt().floor() as usize)).filter(|&x| x % 6 == 1 || x % 6 == 5)
            {
                if p % i == 0 {
                    return false;
                }
            }
            true
        }
    }

    fn inv_mod_p(a: isize, p: isize) -> isize {
        let mut r_pair = (a, p);
        let mut s_pair = (1, 0);
        while r_pair.1 != 0 {
            let quot = r_pair.0 / r_pair.1;
            r_pair = (r_pair.1, r_pair.0 - quot * r_pair.1);
            s_pair = (s_pair.1, s_pair.0 - quot * s_pair.1);
        }
        s_pair.0.rem_euclid(p)
    }
}
