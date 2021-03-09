use std::ops::{Index, IndexMut};

use crate::Polynomial;

impl Index<usize> for Polynomial {
    type Output = isize;
    fn index(&self, index: usize) -> &Self::Output {
        &(self.coeffs[index])
    }
}

impl IndexMut<usize> for Polynomial {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (self.coeffs[index])
    }
}
