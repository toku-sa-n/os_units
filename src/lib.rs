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
//! use os_units::Bytes;
//! use x86_64::structures::paging::{PageSize, Size4KiB};
//!
//! let bytes_of_kernel = Bytes::new(314159);
//! let pages_of_kernel = bytes_of_kernel.as_num_of_pages::<Size4KiB>();
//! assert_eq!(pages_of_kernel.as_usize(), 77);
//!
//! let bytes_of_pages = pages_of_kernel.as_bytes();
//! assert_eq!(bytes_of_pages.as_usize(), 315392);
//! ```
#![no_std]
#![feature(const_fn)]
#![feature(const_fn_fn_ptr_basics)]

use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use x86_64::structures::paging::PageSize;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A struct containing the value with the unit specified by generic type.
pub struct Bytes {
    bytes: usize,
}
impl Bytes {
    /// Creates a new instance with given value.
    pub const fn new(bytes: usize) -> Self {
        Self { bytes }
    }

    /// Returns the value.
    pub const fn as_usize(self) -> usize {
        self.bytes
    }

    /// Converts bytes to the number of physical pages. Note that the number of physical pages will
    /// be calculated so that the specified bytes will be fit in pages.
    pub const fn as_num_of_pages<T: PageSize>(self) -> NumOfPages<T> {
        NumOfPages::new((self.bytes + T::SIZE as usize - 1) / T::SIZE as usize)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumOfPages<T: PageSize> {
    num_of_pages: usize,
    _marker: PhantomData<fn() -> T>,
}
impl<T: PageSize> NumOfPages<T> {
    /// Creates a new instance with given value.
    pub const fn new(num_of_pages: usize) -> Self {
        Self {
            num_of_pages,
            _marker: PhantomData,
        }
    }

    /// Returns the value.
    pub const fn as_usize(self) -> usize {
        self.num_of_pages
    }

    /// Converts the number of physical pages to bytes.
    pub const fn as_bytes(self) -> Bytes {
        Bytes::new(self.num_of_pages * T::SIZE as usize)
    }
}

impl Add<Bytes> for Bytes {
    type Output = Bytes;

    fn add(self, rhs: Bytes) -> Self {
        Self::new(self.bytes + rhs.bytes)
    }
}

impl<T: PageSize> Add<NumOfPages<T>> for NumOfPages<T> {
    type Output = NumOfPages<T>;

    fn add(self, rhs: NumOfPages<T>) -> Self {
        Self::new(self.num_of_pages + rhs.num_of_pages)
    }
}

impl Sub<Bytes> for Bytes {
    type Output = Bytes;

    fn sub(self, rhs: Bytes) -> Self {
        Self::new(self.bytes - rhs.bytes)
    }
}

impl<T: PageSize> Sub<NumOfPages<T>> for NumOfPages<T> {
    type Output = NumOfPages<T>;

    fn sub(self, rhs: NumOfPages<T>) -> Self {
        Self::new(self.num_of_pages - rhs.num_of_pages)
    }
}

impl AddAssign for Bytes {
    fn add_assign(&mut self, rhs: Bytes) {
        self.bytes += rhs.bytes;
    }
}

impl<T: PageSize> AddAssign for NumOfPages<T> {
    fn add_assign(&mut self, rhs: NumOfPages<T>) {
        self.num_of_pages += rhs.num_of_pages;
    }
}

impl SubAssign for Bytes {
    fn sub_assign(&mut self, rhs: Bytes) {
        self.bytes -= rhs.bytes;
    }
}

impl<T: PageSize> SubAssign for NumOfPages<T> {
    fn sub_assign(&mut self, rhs: NumOfPages<T>) {
        self.num_of_pages -= rhs.num_of_pages;
    }
}

impl Mul for Bytes {
    type Output = Bytes;
    fn mul(self, rhs: Bytes) -> Self::Output {
        Self {
            bytes: self.bytes * rhs.bytes,
            ..self
        }
    }
}

impl Mul<usize> for Bytes {
    type Output = Bytes;
    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            bytes: self.bytes * rhs,
            ..self
        }
    }
}

impl<T: PageSize> Mul for NumOfPages<T> {
    type Output = NumOfPages<T>;
    fn mul(self, rhs: NumOfPages<T>) -> Self::Output {
        Self {
            num_of_pages: self.num_of_pages * rhs.num_of_pages,
            ..self
        }
    }
}

impl<T: PageSize> Mul<usize> for NumOfPages<T> {
    type Output = NumOfPages<T>;
    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            num_of_pages: self.num_of_pages * rhs,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use x86_64::structures::paging::{Size1GiB, Size2MiB, Size4KiB};

    #[test]
    fn get_value_from_bytes() {
        let bytes = Bytes::new(334);
        assert_eq!(bytes.as_usize(), 334);
    }

    #[test]
    fn get_value_from_num_of_pages() {
        let pages = NumOfPages::<Size4KiB>::new(334);
        assert_eq!(pages.as_usize(), 334);
    }

    #[test]
    fn bytes_to_pages() {
        let bytes = Bytes::new(0x40000000);
        assert_eq!(bytes.as_num_of_pages::<Size4KiB>().as_usize(), 0x40000);
        assert_eq!(bytes.as_num_of_pages::<Size2MiB>().as_usize(), 512);
        assert_eq!(bytes.as_num_of_pages::<Size1GiB>().as_usize(), 1);
    }

    #[test]
    fn pages_to_bytes_4k() {
        let num_of_pages = NumOfPages::<Size4KiB>::new(1);
        assert_eq!(num_of_pages.as_bytes().as_usize(), 0x1000);
    }

    #[test]
    fn pages_to_bytes_2m() {
        let num_of_pages = NumOfPages::<Size2MiB>::new(1);
        assert_eq!(num_of_pages.as_bytes().as_usize(), 0x200000);
    }

    #[test]
    fn pages_to_bytes_1g() {
        let num_of_pages = NumOfPages::<Size1GiB>::new(1);
        assert_eq!(num_of_pages.as_bytes().as_usize(), 0x40000000);
    }

    #[test]
    fn addition_bytes_to_bytes() {
        let b1 = Bytes::new(3);
        let b2 = Bytes::new(1);
        let sum = b1 + b2;

        assert_eq!(sum.as_usize(), 4);
    }

    #[test]
    fn addition_pages_to_pages() {
        let p1 = NumOfPages::<Size4KiB>::new(3);
        let p2 = NumOfPages::<Size4KiB>::new(1);
        let sum = p1 + p2;

        assert_eq!(sum.as_usize(), 4);
    }

    #[test]
    fn subtraction_bytes_from_bytes() {
        let b1 = Bytes::new(3);
        let b2 = Bytes::new(1);
        let diff = b1 - b2;

        assert_eq!(diff.as_usize(), 2);
    }

    #[test]
    fn subtraction_pages_from_pages() {
        let p1 = NumOfPages::<Size4KiB>::new(3);
        let p2 = NumOfPages::<Size4KiB>::new(1);
        let diff = p1 - p2;

        assert_eq!(diff.as_usize(), 2);
    }

    #[test]
    fn add_assign_bytes_to_bytes() {
        let mut b1 = Bytes::new(3);
        b1 += Bytes::new(1);

        assert_eq!(b1.as_usize(), 4);
    }

    #[test]
    fn add_assign_pages_to_pages() {
        let mut p1 = NumOfPages::<Size4KiB>::new(3);
        p1 += NumOfPages::<Size4KiB>::new(1);

        assert_eq!(p1.as_usize(), 4);
    }

    #[test]
    fn sub_assign_bytes_to_bytes() {
        let mut b1 = Bytes::new(3);
        b1 -= Bytes::new(1);

        assert_eq!(b1.as_usize(), 2);
    }

    #[test]
    fn sub_assign_pages_to_pages() {
        let mut p1 = NumOfPages::<Size4KiB>::new(3);
        p1 -= NumOfPages::<Size4KiB>::new(1);

        assert_eq!(p1.as_usize(), 2);
    }

    #[test]
    fn mul_from_bytes_to_bytes() {
        let b1 = Bytes::new(3);
        let b2 = Bytes::new(4);
        let mul = b1 * b2;

        assert_eq!(mul.as_usize(), 12);
    }

    #[test]
    fn mul_from_pages_to_pages() {
        let p1 = NumOfPages::<Size4KiB>::new(3);
        let p2 = NumOfPages::<Size4KiB>::new(4);
        let mul = p1 * p2;

        assert_eq!(mul.as_usize(), 12);
    }

    #[test]
    fn mul_bytes_by_usize() {
        let b = Bytes::new(3);
        let mul = b * 4;

        assert_eq!(mul.as_usize(), 12);
    }

    #[test]
    fn mul_pages_by_usize() {
        let p = NumOfPages::<Size4KiB>::new(3);
        let mul = p * 4;

        assert_eq!(mul.as_usize(), 12);
    }
}
