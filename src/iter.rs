use crate::Polynomial;

impl IntoIterator for Polynomial {
    type Item = isize;
    type IntoIter = std::vec::IntoIter<isize>;

    fn into_iter(self) -> Self::IntoIter {
        self.coeffs.into_iter()
    }
}

impl Polynomial {
    /// Returns an immutably referencing iterator over the underlying
    /// `Vec` of coefficients.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let poly = poly![5, 3, -2, 1];
    /// let mut iter = poly.iter();
    ///
    /// assert_eq!(iter.next(), Some(&5));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&(-2)));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&'_ self) -> std::slice::Iter<'_, isize> {
        self.coeffs.iter()
    }

    /// Returns a mutably referencing iterator over the underlying
    /// `Vec` of coefficients.
    ///
    /// # Examples
    /// ```
    /// use polynomint::{Polynomial, poly};
    ///
    /// let mut poly = poly![5, 3, -2, 1];
    /// for coeff in poly.iter_mut() {
    ///     *coeff *= 3;
    /// }
    /// assert_eq!(poly, poly![15, 9, -6, 3]);
    /// ```
    pub fn iter_mut(&'_ mut self) -> std::slice::IterMut<'_, isize> {
        self.coeffs.iter_mut()
    }
}
