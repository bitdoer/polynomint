pub mod add;
pub mod mul;
pub mod rem;
pub mod sub;

/// A wrapper struct around a `Vec<isize>` which treats the entries of the `Vec` as the coefficients
/// of a polynomial.
///
/// # Examples
/// ```
/// use polynomint::{Polynomial, poly};
///
/// let quadratic = poly![1, 2, 1]; // x^2 + 2x + 1
/// let linear = poly![-6, 1]; // x - 6
/// assert_eq!(&quadratic * 5, poly![5, 10, 5]);
/// assert_eq!(&quadratic * &linear, poly![-6, -11, -4, 1]);
///
/// let mut resultant = &quadratic * &linear;
/// resultant %= 5;
/// assert_eq!(resultant, poly![-1, -1, -4, 1]);
///
/// let resultant2 = (&quadratic * &linear).rem_euclid(5);
/// assert_eq!(resultant2, poly![4, 4, 1, 1]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    coeffs: Vec<isize>,
}

impl Polynomial {
    /// Creates a polynomial with the given coefficients, stored in increasing order, with
    /// any trailing (higher-degree) zeroes removed.
    ///
    /// # Examples
    /// ```
    /// use polynomint::Polynomial;
    ///
    /// let quadratic = Polynomial::new(vec![1, 2, 3]); // 3x^2 + 2x + 1
    /// let cubic = Polynomial::new(vec![8, 12, 6, 1]); // x^3 + 6x^2 + 12x + 8
    /// ```
    pub fn new(coeffs: Vec<isize>) -> Self {
        let mut output = Self { coeffs };
        output.reduce();
        output
    }

    /// Creates the zero polynomial, which is stored internally as an empty vector.
    pub fn zero() -> Self {
        Self { coeffs: Vec::new() }
    }

    /// Creates a constant polynomial with coefficient equal to the argument passed;
    /// if the argument passed is zero, it is stored internally as an empty vector
    /// to match `zero()`.
    pub fn constant(i: isize) -> Self {
        if i == 0 {
            Self::zero()
        } else {
            Self { coeffs: vec![i] }
        }
    }
    /// Gives the highest power which has a nonzero coefficient; constants are degree zero, except
    /// the constant polynomial 0, which has degree -1.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let zero = Polynomial::zero();
    /// let alt_zero = Polynomial::constant(0);
    /// let three = Polynomial::constant(3);
    /// let classic = poly![1, 2, 1, 0, 0]; // x^2 + 2x + 1
    ///
    /// assert_eq!(zero.degree(), -1);
    /// assert_eq!(alt_zero.degree(), -1);
    /// assert_eq!(three.degree(), 0);
    /// assert_eq!(classic.degree(), 2);
    /// ```
    pub fn degree(&self) -> isize {
        (self.coeffs.len() as isize) - 1
    }

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
    /// Checks whether a polynomial is the zero polynomial.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let zero = Polynomial::zero();
    /// let also_zero = Polynomial::constant(0);
    /// let yet_again_zero = poly![0, 0, 0, 0];
    /// let even_more_zero = poly![1, 2, 1] - poly![1, 2, 1];
    /// let not_zero = poly![0, 1];
    ///
    /// assert!(zero.is_zero());
    /// assert!(also_zero.is_zero());
    /// assert!(yet_again_zero.is_zero());
    /// assert!(even_more_zero.is_zero());
    /// assert!(!not_zero.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.degree() == -1
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

    /// Removes trailing zeroes from a polynomial. Used to make sure the API only exposes
    /// polynomials with no stored zeroes of higher-order, both to keep them as lightweight
    /// as possible and because this invariant is taken advantage of by functions like
    /// degree().
    fn reduce(&mut self) {
        while self.coeffs.last() == Some(&0) {
            self.coeffs.pop();
        }
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        // if our polynomial is zero, the big-ass else block doesn't do anything, so
        // we just separately handle that case
        if self.is_zero() {
            s += &format!("{}", 0);
        } else {
            // this flag is basically "have we written a term yet", so we know whether
            // to write plus/minus signs and to treat minus signs as operations rather
            // than as prefixes
            let mut plus_flag = false;
            for (n, &i) in self.coeffs.iter().enumerate().rev() {
                // only display terms with nonzero coefficients
                if i != 0 {
                    // if we're past the first term, print the appropriate operation
                    // sign first
                    if plus_flag {
                        if i < 0 {
                            s += " - ";
                        } else {
                            s += " + "
                        }
                    }
                    // if our term is constant,
                    if n == 0 {
                        // just display that constant, or its absolute value if we already
                        // wrote a minus sign
                        s += &format!("{}", if plus_flag { i.abs() } else { i });
                    // if our term is linear,
                    } else if n == 1 {
                        // if it's 1, just write "x";
                        if i == 1 {
                            s += "x";
                        // if it's -1, and if we're writing the first term, put a minus sign
                        // in front
                        } else if i == -1 {
                            s += &format!("{}x", if plus_flag { "" } else { "-" });
                        // otherwise, just display the coefficient, or its absolute value
                        // if we already wrote a minus sign
                        } else {
                            s += &format!("{}x", if plus_flag { i.abs() } else { i });
                        }
                    // rest of cases as above, but with the powers being displayed as well
                    } else if i == 1 {
                        s += &format!("x^{}", n);
                    } else if i == -1 {
                        s += &format!("{}x^{}", if plus_flag { "" } else { "-" }, n);
                    } else {
                        s += &format!("{}x^{}", if plus_flag { i.abs() } else { i }, n);
                    }
                    plus_flag = true;
                }
            }
        }
        write!(f, "{}", s)
    }
}

/// A convenience macro for writing polynomials; essentially a wrapper around `vec![...]`.
#[macro_export]
macro_rules! poly {
    () => (
        Polynomial::zero();
    );
    ($($x:expr),*) => (
        Polynomial::new(vec![$($x),*]);
    )
}

#[cfg(test)]
mod tests {
    use crate::{poly, Polynomial};
    #[test]
    fn it_works() {
        let mut quadratic = poly![1, 2, 1]; // x^2 + 2x + 1
        let linear = poly![-6, 1]; // x - 6
        assert_eq!(&quadratic + &linear, poly![-5, 3, 1]);
        assert_eq!(&quadratic - &linear, poly![7, 1, 1]);
        assert_eq!(&quadratic * &linear, poly![-6, -11, -4, 1]);
        quadratic -= &linear;
        assert_eq!(quadratic, poly![7, 1, 1]);
        quadratic += &linear;
        assert_eq!(quadratic, poly![1, 2, 1]);
        quadratic *= &linear;
        assert_eq!(quadratic, poly![-6, -11, -4, 1]);
        assert_eq!(quadratic.derivative(), poly![-11, -8, 3]);
        let mut another = poly![1, 3, 3, 1]; // x^3 + 3x^2 + 3x + 1
        let pair = poly![-5, 4, 2]; // 2x^2 + 4x - 5
        assert_eq!(&another + &pair, poly![-4, 7, 5, 1]);
        assert_eq!(&another - &pair, poly![6, -1, 1, 1]);
        assert_eq!(&another * &pair, poly![-5, -11, -1, 13, 10, 2]);
        another -= &pair;
        assert_eq!(another, poly![6, -1, 1, 1]);
        another += &pair;
        assert_eq!(another, poly![1, 3, 3, 1]);
        another *= &pair;
        assert_eq!(another, poly![-5, -11, -1, 13, 10, 2]);
        assert_eq!(another.derivative(), poly![-11, -2, 39, 40, 10]);
    }
}
