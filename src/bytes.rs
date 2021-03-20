use crate::NumOfPages;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Sub;
use core::ops::SubAssign;
use x86_64::structures::paging::PageSize;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    fn bytes_to_pages() {
        let bytes = Bytes::new(0x40000000);
        assert_eq!(bytes.as_num_of_pages::<Size4KiB>().as_usize(), 0x40000);
        assert_eq!(bytes.as_num_of_pages::<Size2MiB>().as_usize(), 512);
        assert_eq!(bytes.as_num_of_pages::<Size1GiB>().as_usize(), 1);
    }

    #[test]
    fn addition_bytes_to_bytes() {
        let b1 = Bytes::new(3);
        let b2 = Bytes::new(1);
        let sum = b1 + b2;

        assert_eq!(sum.as_usize(), 4);
    }

    #[test]
    fn add_usize_to_bytes() {
        let b = Bytes::new(3);

        assert_eq!(b + 7, Bytes::new(10));
    }

    #[test]
    fn subtraction_bytes_from_bytes() {
        let b1 = Bytes::new(3);
        let b2 = Bytes::new(1);
        let diff = b1 - b2;

        assert_eq!(diff.as_usize(), 2);
    }

    #[test]
    fn subtract_usize_from_bytes() {
        let b = Bytes::new(5);

        assert_eq!(b - 3, Bytes::new(2));
    }

    #[test]
    fn add_assign_bytes_to_bytes() {
        let mut b1 = Bytes::new(3);
        b1 += Bytes::new(1);

        assert_eq!(b1.as_usize(), 4);
    }

    #[test]
    fn add_assign_usize_to_bytes() {
        let mut b1 = Bytes::new(3);
        b1 += 1;

        assert_eq!(b1.as_usize(), 4);
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
    fn mul_bytes_by_usize() {
        let b = Bytes::new(3);
        let mul = b * 4;

        assert_eq!(mul.as_usize(), 12);
    }

    #[test]
    fn mul_assign_bytes_by_usize() {
        let mut b = Bytes::new(3);
        b *= 4;

        assert_eq!(b.as_usize(), 12);
    }

    #[test]
    fn div_bytes_by_usize() {
        let b1 = Bytes::new(3);
        let div = b1 / 2;

        assert_eq!(div.as_usize(), 1);
    }

    #[test]
    fn divassign_bytes_by_usize() {
        let mut b = Bytes::new(3);
        b /= 2;

        assert_eq!(b.as_usize(), 1);
    }

    #[test]
    fn bytes_zero() {
        let b = Bytes::zero();

        assert_eq!(b.as_usize(), 0);
    }
}
