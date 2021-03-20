use crate::Bytes;
use core::marker::PhantomData;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Div;
use core::ops::DivAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use core::ops::Sub;
use core::ops::SubAssign;
use x86_64::structures::paging::PageSize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
