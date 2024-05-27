//! Ergonomic array slice for 2d array manipulation.
//!
//! ## Example
//!
//! ```
//! # use slice2d::Slice2DExt;
//! let array = [1, 2, 3, 4, 5, 6];
//! let slice = array.get_slice2d(3, 2).unwrap();
//! assert_eq!(slice[2][0], 5);
//! assert_eq!(&slice[1], &[3, 4]);
//! ```
#![no_std]
use core::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A 2 dimensional slice.
pub struct Slice2D<'t, T> {
    stride: usize,
    len: usize,
    slice: &'t [T],
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A mutable 2 dimensional slice.
pub struct Slice2DMut<'t, T> {
    stride: usize,
    len: usize,
    slice: &'t mut [T],
}

impl<T> Slice2D<'_, T> {
    /// Returns a reference to a subslice.
    pub fn get(&self, index: usize) -> Option<&[T]> {
        let origin = index * self.stride;
        self.slice.get(origin..origin + self.len)
    }
}

impl<'t, T> Slice2DMut<'t, T> {
    /// Obtain a [`Slice2D`] from a [`Slice2DMut`].
    pub fn downgrade(&self) -> Slice2D<'_, T> {
        Slice2D {
            stride: self.stride,
            len: self.len,
            slice: self.slice,
        }
    }

    /// Cast [`Slice2DMut`] to a new lifetime.
    pub fn reborrow(&mut self) -> Slice2DMut<'_, T> {
        Slice2DMut {
            stride: self.stride,
            len: self.len,
            slice: self.slice,
        }
    }

    /// Returns a reference to a subslice.
    pub fn get(&self, index: usize) -> Option<&[T]> {
        let origin = index * self.stride;
        self.slice.get(origin..origin + self.len)
    }

    /// Returns a mutable reference to a subslice.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut [T]> {
        let origin = index * self.stride;
        self.slice.get_mut(origin..origin + self.len)
    }
}

impl<T> Index<usize> for Slice2D<'_, T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let origin = index * self.stride;
        &self.slice[origin..origin + self.len]
    }
}

impl<T> Index<usize> for Slice2DMut<'_, T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let origin = index * self.stride;
        &self.slice[origin..origin + self.len]
    }
}

impl<T> IndexMut<usize> for Slice2DMut<'_, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let origin = index * self.stride;
        &mut self.slice[origin..origin + self.len]
    }
}

impl<'t, T> Iterator for Slice2D<'t, T> {
    type Item = &'t [T];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.slice.get(0..self.len);
        self.slice = self.slice.get(self.stride..).unwrap_or(&[]);
        result
    }
}

/// Extension for creating 2 dimensional slices.
pub trait Slice2DExt {
    type Item;

    /// Obtain a [`Slice2D`] with row length `len` and skips `stride` per row.
    ///
    /// Incomplete rows will be discarded.
    fn slice2d(&self, len: usize, stride: usize) -> Slice2D<'_, Self::Item>;
    /// Obtain a [`Slice2DMut`] with row length `len` and skips `stride` per row.
    ///
    /// Incomplete rows will be discarded.
    fn slice2d_mut(&mut self, len: usize, stride: usize) -> Slice2DMut<'_, Self::Item>;

    /// Obtain a [`Slice2D`] by validating dimension.
    ///
    /// This is equivalent to calling `slice2d(y, y)`.
    fn get_slice2d(&self, x: usize, y: usize) -> Option<Slice2D<'_, Self::Item>>;
    /// Obtain a [`Slice2DMut`] by validating dimension.
    ///
    /// This is equivalent to calling `slice2d_mut(y, y)`.
    fn get_slice2d_mut(&mut self, x: usize, y: usize) -> Option<Slice2DMut<'_, Self::Item>>;
}

impl<T> Slice2DExt for [T] {
    type Item = T;
    fn slice2d(&self, len: usize, stride: usize) -> Slice2D<'_, Self::Item> {
        Slice2D {
            len,
            stride,
            slice: self,
        }
    }

    fn slice2d_mut(&mut self, len: usize, stride: usize) -> Slice2DMut<'_, Self::Item> {
        Slice2DMut {
            len,
            stride,
            slice: self,
        }
    }

    fn get_slice2d(&self, x: usize, y: usize) -> Option<Slice2D<'_, Self::Item>> {
        if self.len() != x * y {
            return None;
        }
        Some(Slice2D {
            len: y,
            stride: y,
            slice: self,
        })
    }

    fn get_slice2d_mut(&mut self, x: usize, y: usize) -> Option<Slice2DMut<'_, Self::Item>> {
        if self.len() != x * y {
            return None;
        }
        Some(Slice2DMut {
            len: y,
            stride: y,
            slice: self,
        })
    }
}

#[cfg(test)]
mod test {
    pub use crate::Slice2DExt;

    #[test]
    fn test() {
        let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut slice = v.get_slice2d(3, 3).unwrap();
        assert_eq!(slice[0][0], 1);
        assert_eq!(slice[0][1], 2);
        assert_eq!(slice[0][2], 3);
        assert_eq!(slice[1][0], 4);
        assert_eq!(slice[1][1], 5);
        assert_eq!(slice[1][2], 6);
        assert_eq!(slice[2][0], 7);
        assert_eq!(slice[2][1], 8);
        assert_eq!(slice[2][2], 9);
        assert_eq!(&slice[1], &[4, 5, 6]);

        assert_eq!(slice.next().unwrap(), &[1, 2, 3]);
        assert_eq!(slice.next().unwrap(), &[4, 5, 6]);
        assert_eq!(slice.next().unwrap(), &[7, 8, 9]);
        assert_eq!(slice.next(), None);
        assert_eq!(slice.next(), None);

        let mut v = [1, 2, 3, 4, 5, 6];
        let slice = v.get_slice2d_mut(3, 2).unwrap();
        assert_eq!(slice[0][0], 1);
        assert_eq!(slice[0][1], 2);
        assert_eq!(slice[1][0], 3);
        assert_eq!(slice[1][1], 4);
        assert_eq!(slice[2][0], 5);
        assert_eq!(slice[2][1], 6);
        assert_eq!(&slice[1], &[3, 4]);
    }
}
