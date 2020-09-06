// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This crate provides a data structure for byte size. With
//! [`x86_64`](https://github.com/rust-osdev/x86_64) crate, you can easily convert
//! the size of physical memory pages into bytes, and bytes into the number of physical memory
//! pages.
//!
//! Currently, this crate only supports Rust nightly version because of using
//! [`const_fn`](https://doc.rust-lang.org/beta/unstable-book/language-features/const-fn.html) feature.
//!
//! # Examples
//!
//! ```rust
//! use os_units::{Size, Bytes};
//! use x86_64::structures::paging::{PageSize, Size4KiB};
//!
//! let bytes_of_kernel = Size::<Bytes>::new(314159);
//! let pages_of_kernel = bytes_of_kernel.as_num_of_pages::<Size4KiB>();
//! assert_eq!(pages_of_kernel.as_usize(), 77);
//!
//! let bytes_of_pages = pages_of_kernel.as_bytes();
//! assert_eq!(bytes_of_pages.as_usize(), 315392);
//! ```
#![no_std]
#![feature(const_fn)]

use core::marker::PhantomData;
use core::ops::Add;
use core::ops::Sub;
use x86_64::structures::paging::PageSize;

/// A marker trait for representing units.
pub trait Unit: Copy + Clone + PartialEq + Eq + PartialOrd + Ord {}

/// A struct representing bytes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes;
impl Unit for Bytes {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A struct representing the number of physical pages.
pub struct NumOfPages<T: PageSize> {
    _marker: PhantomData<fn() -> T>,
}
impl<T: PageSize> Unit for NumOfPages<T> {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A struct containing the value with the unit specified by generic type.
pub struct Size<T: Unit> {
    val: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T: Unit> Size<T> {
    /// Creates a new instance with given value.
    pub const fn new(val: usize) -> Self {
        Self {
            val,
            _marker: PhantomData,
        }
    }

    /// Returns the value.
    pub const fn as_usize(self) -> usize {
        self.val
    }
}

impl Size<Bytes> {
    /// Converts bytes to the number of physical pages. Note that the number of physical pages will
    /// be calculated so that the specified bytes will be fit in pages.
    pub const fn as_num_of_pages<T: PageSize>(self) -> Size<NumOfPages<T>> {
        Size {
            val: (self.val + T::SIZE as usize - 1) / T::SIZE as usize,
            _marker: PhantomData,
        }
    }
}

impl<T: PageSize> Size<NumOfPages<T>> {
    /// Convert the number of physical pages to bytes.
    pub const fn as_bytes(self) -> Size<Bytes> {
        Size {
            val: self.val * T::SIZE as usize,
            _marker: PhantomData,
        }
    }
}

impl Add<Size<Bytes>> for Size<Bytes> {
    type Output = Size<Bytes>;

    fn add(self, rhs: Size<Bytes>) -> Self {
        Self::new(self.val + rhs.val)
    }
}

impl<T: PageSize> Add<Size<NumOfPages<T>>> for Size<NumOfPages<T>> {
    type Output = Size<NumOfPages<T>>;

    fn add(self, rhs: Size<NumOfPages<T>>) -> Self {
        Self::new(self.val + rhs.val)
    }
}

impl Sub<Size<Bytes>> for Size<Bytes> {
    type Output = Size<Bytes>;

    fn sub(self, rhs: Size<Bytes>) -> Self {
        Self::new(self.val - rhs.val)
    }
}

impl<T: PageSize> Sub<Size<NumOfPages<T>>> for Size<NumOfPages<T>> {
    type Output = Size<NumOfPages<T>>;

    fn sub(self, rhs: Size<NumOfPages<T>>) -> Self {
        Self::new(self.val - rhs.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use x86_64::structures::paging::{Size1GiB, Size2MiB, Size4KiB};

    #[test]
    fn get_value_from_bytes() {
        let bytes = Size::<Bytes>::new(334);
        assert_eq!(bytes.as_usize(), 334);
    }

    #[test]
    fn get_value_from_num_of_pages() {
        let pages = Size::<NumOfPages<Size4KiB>>::new(334);
        assert_eq!(pages.as_usize(), 334);
    }

    #[test]
    fn bytes_to_pages() {
        let bytes = Size::<Bytes>::new(0x40000000);
        assert_eq!(bytes.as_num_of_pages::<Size4KiB>().as_usize(), 0x40000);
        assert_eq!(bytes.as_num_of_pages::<Size2MiB>().as_usize(), 512);
        assert_eq!(bytes.as_num_of_pages::<Size1GiB>().as_usize(), 1);
    }

    #[test]
    fn pages_to_bytes_4k() {
        let num_of_pages = Size::<NumOfPages<Size4KiB>>::new(1);
        assert_eq!(num_of_pages.as_bytes().as_usize(), 0x1000);
    }

    #[test]
    fn pages_to_bytes_2m() {
        let num_of_pages = Size::<NumOfPages<Size2MiB>>::new(1);
        assert_eq!(num_of_pages.as_bytes().as_usize(), 0x200000);
    }

    #[test]
    fn pages_to_bytes_1g() {
        let num_of_pages = Size::<NumOfPages<Size1GiB>>::new(1);
        assert_eq!(num_of_pages.as_bytes().as_usize(), 0x40000000);
    }

    #[test]
    fn addition_bytes_to_bytes() {
        let b1 = Size::<Bytes>::new(3);
        let b2 = Size::<Bytes>::new(1);
        let sum = b1 + b2;

        assert_eq!(sum.as_usize(), 4);
    }

    #[test]
    fn addition_pages_to_pages() {
        let p1 = Size::<NumOfPages<Size4KiB>>::new(3);
        let p2 = Size::<NumOfPages<Size4KiB>>::new(1);
        let sum = p1 + p2;

        assert_eq!(sum.as_usize(), 4);
    }

    #[test]
    fn subtraction_bytes_from_bytes() {
        let b1 = Size::<Bytes>::new(3);
        let b2 = Size::<Bytes>::new(1);
        let diff = b1 - b2;

        assert_eq!(diff.as_usize(), 2);
    }

    #[test]
    fn subtraction_pages_from_pages() {
        let p1 = Size::<NumOfPages<Size4KiB>>::new(3);
        let p2 = Size::<NumOfPages<Size4KiB>>::new(1);
        let diff = p1 - p2;

        assert_eq!(diff.as_usize(), 2);
    }
}
