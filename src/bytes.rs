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
