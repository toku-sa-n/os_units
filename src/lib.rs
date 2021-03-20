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
#![deny(
    warnings,
    rustdoc::all,
    missing_docs,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    missing_copy_implementations,
    meta_variable_misuse,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    unaligned_references,
    unreachable_pub,
    unused_crate_dependencies,
    unused_extern_crates,
    trivial_casts,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    pointer_structural_match,
    missing_debug_implementations
)]
#![deny(clippy::all, clippy::pedantic)]

use {
    core::{
        marker::PhantomData,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    },
    x86_64::structures::paging::PageSize,
};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A struct representing byte size.
pub struct Bytes(usize);
impl Bytes {
    /// Creates a new instance with given value.
    #[must_use]
    pub const fn new(bytes: usize) -> Self {
        Self(bytes)
    }

    /// Equivalent to `Bytes::new(0)`.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0)
    }

    /// Returns the value.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Converts bytes to the number of physical pages. Note that the number of physical pages will
    /// be calculated so that the specified bytes will be fit in pages.
    #[must_use]
    pub const fn as_num_of_pages<T: PageSize>(self) -> NumOfPages<T> {
        #[allow(clippy::cast_possible_truncation)]
        NumOfPages::new((self.0 + T::SIZE as usize - 1) / T::SIZE as usize)
    }
}
impl Add for Bytes {
    type Output = Bytes;

    fn add(self, rhs: Bytes) -> Self {
        Self::new(self.0 + rhs.0)
    }
}
impl Add<usize> for Bytes {
    type Output = Bytes;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}
impl AddAssign for Bytes {
    fn add_assign(&mut self, rhs: Bytes) {
        self.0 += rhs.0;
    }
}
impl AddAssign<usize> for Bytes {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
impl Sub for Bytes {
    type Output = Bytes;

    fn sub(self, rhs: Bytes) -> Self {
        Self::new(self.0 - rhs.0)
    }
}
impl Sub<usize> for Bytes {
    type Output = Bytes;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.0 - rhs)
    }
}
impl SubAssign for Bytes {
    fn sub_assign(&mut self, rhs: Bytes) {
        self.0 -= rhs.0;
    }
}
impl SubAssign<usize> for Bytes {
    fn sub_assign(&mut self, rhs: usize) {
        *self -= Bytes::new(rhs);
    }
}
impl Mul<usize> for Bytes {
    type Output = Bytes;
    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl MulAssign<usize> for Bytes {
    fn mul_assign(&mut self, rhs: usize) {
        *self = *self * rhs;
    }
}
impl Div<usize> for Bytes {
    type Output = Bytes;

    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}
impl DivAssign<usize> for Bytes {
    fn div_assign(&mut self, rhs: usize) {
        *self = *self / rhs;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A struct representing the number of pages.
pub struct NumOfPages<T: PageSize> {
    num_of_pages: usize,
    _marker: PhantomData<fn() -> T>,
}
impl<T: PageSize> NumOfPages<T> {
    /// Creates a new instance with given value.
    #[must_use]
    pub const fn new(num_of_pages: usize) -> Self {
        Self {
            num_of_pages,
            _marker: PhantomData,
        }
    }

    /// Equivalent to `NumOfPages::new(0)`.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0)
    }

    /// Returns the value.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.num_of_pages
    }

    /// Converts the number of physical pages to bytes.
    #[must_use]
    pub const fn as_bytes(self) -> Bytes {
        #[allow(clippy::cast_possible_truncation)]
        Bytes::new(self.num_of_pages * T::SIZE as usize)
    }
}
impl<T: PageSize> Add for NumOfPages<T> {
    type Output = NumOfPages<T>;

    fn add(self, rhs: NumOfPages<T>) -> Self {
        Self::new(self.num_of_pages + rhs.num_of_pages)
    }
}
impl<T: PageSize> Add<usize> for NumOfPages<T> {
    type Output = NumOfPages<T>;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.num_of_pages + rhs)
    }
}
impl<T: PageSize> AddAssign for NumOfPages<T> {
    fn add_assign(&mut self, rhs: NumOfPages<T>) {
        self.num_of_pages += rhs.num_of_pages;
    }
}
impl<T: PageSize> AddAssign<usize> for NumOfPages<T> {
    fn add_assign(&mut self, rhs: usize) {
        self.num_of_pages += rhs;
    }
}
impl<T: PageSize> Sub for NumOfPages<T> {
    type Output = NumOfPages<T>;

    fn sub(self, rhs: NumOfPages<T>) -> Self {
        Self::new(self.num_of_pages - rhs.num_of_pages)
    }
}
impl<T: PageSize> Sub<usize> for NumOfPages<T> {
    type Output = NumOfPages<T>;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(self.num_of_pages - rhs)
    }
}
impl<T: PageSize> SubAssign for NumOfPages<T> {
    fn sub_assign(&mut self, rhs: NumOfPages<T>) {
        self.num_of_pages -= rhs.num_of_pages;
    }
}
impl<T: PageSize> SubAssign<usize> for NumOfPages<T> {
    fn sub_assign(&mut self, rhs: usize) {
        *self -= Self::new(rhs);
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
impl<T: PageSize> MulAssign<usize> for NumOfPages<T> {
    fn mul_assign(&mut self, rhs: usize) {
        *self = *self * rhs;
    }
}
impl<T: PageSize> Div<usize> for NumOfPages<T> {
    type Output = NumOfPages<T>;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            num_of_pages: self.num_of_pages / rhs,
            ..self
        }
    }
}
impl<T: PageSize> DivAssign<usize> for NumOfPages<T> {
    fn div_assign(&mut self, rhs: usize) {
        *self = *self / rhs;
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
    fn add_usize_to_bytes() {
        let b = Bytes::new(3);

        assert_eq!(b + 7, Bytes::new(10));
    }

    #[test]
    fn add_usize_to_num_of_pages() {
        let n = NumOfPages::<Size4KiB>::new(3);

        assert_eq!(n + 7, NumOfPages::new(10));
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
    fn subtract_usize_from_bytes() {
        let b = Bytes::new(5);

        assert_eq!(b - 3, Bytes::new(2));
    }

    #[test]
    fn subtract_usize_from_num_of_pages() {
        let n = NumOfPages::<Size4KiB>::new(5);

        assert_eq!(n - 3, NumOfPages::new(2));
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
    fn add_assign_usize_to_bytes() {
        let mut b1 = Bytes::new(3);
        b1 += 1;

        assert_eq!(b1.as_usize(), 4);
    }

    #[test]
    fn add_assign_usize_to_pages() {
        let mut p1 = NumOfPages::<Size4KiB>::new(3);
        p1 += 1;

        assert_eq!(p1.as_usize(), 4);
    }

    #[test]
    fn sub_assign_bytes_to_bytes() {
        let mut b1 = Bytes::new(3);
        b1 -= Bytes::new(1);

        assert_eq!(b1.as_usize(), 2);
    }

    #[test]
    fn sub_assign_usize_to_bytes() {
        let mut b1 = Bytes::new(10);
        b1 -= 3;

        assert_eq!(b1, Bytes::new(7));
    }

    #[test]
    fn sub_assign_pages_to_pages() {
        let mut p1 = NumOfPages::<Size4KiB>::new(3);
        p1 -= NumOfPages::<Size4KiB>::new(1);

        assert_eq!(p1.as_usize(), 2);
    }

    #[test]
    fn sub_assign_usize_to_num_of_pages() {
        let mut p1 = NumOfPages::<Size4KiB>::new(10);
        p1 -= 3;

        assert_eq!(p1, NumOfPages::new(7));
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

    #[test]
    fn mul_assign_bytes_by_usize() {
        let mut b = Bytes::new(3);
        b *= 4;

        assert_eq!(b.as_usize(), 12);
    }

    #[test]
    fn mul_assign_pages_by_usize() {
        let mut p = NumOfPages::<Size4KiB>::new(3);
        p *= 4;

        assert_eq!(p.as_usize(), 12);
    }

    #[test]
    fn div_bytes_by_usize() {
        let b1 = Bytes::new(3);
        let div = b1 / 2;

        assert_eq!(div.as_usize(), 1);
    }

    #[test]
    fn div_num_of_pages_by_usize() {
        let p1 = NumOfPages::<Size4KiB>::new(3);
        let div = p1 / 2;

        assert_eq!(div.as_usize(), 1);
    }

    #[test]
    fn divassign_bytes_by_usize() {
        let mut b = Bytes::new(3);
        b /= 2;

        assert_eq!(b.as_usize(), 1);
    }

    #[test]
    fn divassign_num_of_pages_by_usize() {
        let mut p = NumOfPages::<Size4KiB>::new(3);
        p /= 2;

        assert_eq!(p.as_usize(), 1);
    }

    #[test]
    fn bytes_zero() {
        let b = Bytes::zero();

        assert_eq!(b.as_usize(), 0);
    }

    #[test]
    fn num_of_pages_zero() {
        let n = NumOfPages::<Size4KiB>::zero();

        assert_eq!(n.as_usize(), 0);
    }
}
