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
impl<T: PageSize> From<usize> for NumOfPages<T> {
    fn from(n: usize) -> Self {
        Self::new(n)
    }
}

#[cfg(test)]
mod tests {
    use super::NumOfPages;
    use x86_64::structures::paging::Size1GiB;
    use x86_64::structures::paging::Size2MiB;
    use x86_64::structures::paging::Size4KiB;

    #[test]
    fn get_value_from_num_of_pages() {
        let pages = NumOfPages::<Size4KiB>::new(334);
        assert_eq!(pages.as_usize(), 334);
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
    fn addition_pages_to_pages() {
        let p1 = NumOfPages::<Size4KiB>::new(3);
        let p2 = NumOfPages::<Size4KiB>::new(1);
        let sum = p1 + p2;

        assert_eq!(sum.as_usize(), 4);
    }

    #[test]
    fn add_usize_to_num_of_pages() {
        let n = NumOfPages::<Size4KiB>::new(3);

        assert_eq!(n + 7, NumOfPages::new(10));
    }

    #[test]
    fn subtraction_pages_from_pages() {
        let p1 = NumOfPages::<Size4KiB>::new(3);
        let p2 = NumOfPages::<Size4KiB>::new(1);
        let diff = p1 - p2;

        assert_eq!(diff.as_usize(), 2);
    }

    #[test]
    fn subtract_usize_from_num_of_pages() {
        let n = NumOfPages::<Size4KiB>::new(5);

        assert_eq!(n - 3, NumOfPages::new(2));
    }

    #[test]
    fn add_assign_pages_to_pages() {
        let mut p1 = NumOfPages::<Size4KiB>::new(3);
        p1 += NumOfPages::<Size4KiB>::new(1);

        assert_eq!(p1.as_usize(), 4);
    }

    #[test]
    fn add_assign_usize_to_pages() {
        let mut p1 = NumOfPages::<Size4KiB>::new(3);
        p1 += 1;

        assert_eq!(p1.as_usize(), 4);
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
    fn mul_pages_by_usize() {
        let p = NumOfPages::<Size4KiB>::new(3);
        let mul = p * 4;

        assert_eq!(mul.as_usize(), 12);
    }

    #[test]
    fn mul_assign_pages_by_usize() {
        let mut p = NumOfPages::<Size4KiB>::new(3);
        p *= 4;

        assert_eq!(p.as_usize(), 12);
    }

    #[test]
    fn div_num_of_pages_by_usize() {
        let p1 = NumOfPages::<Size4KiB>::new(3);
        let div = p1 / 2;

        assert_eq!(div.as_usize(), 1);
    }

    #[test]
    fn divassign_num_of_pages_by_usize() {
        let mut p = NumOfPages::<Size4KiB>::new(3);
        p /= 2;

        assert_eq!(p.as_usize(), 1);
    }

    #[test]
    fn num_of_pages_zero() {
        let n = NumOfPages::<Size4KiB>::zero();

        assert_eq!(n.as_usize(), 0);
    }

    #[test]
    fn from() {
        let n = NumOfPages::<Size4KiB>::from(3);

        assert_eq!(n, NumOfPages::new(3));
    }

    #[test]
    fn debug() {
        let n = NumOfPages::<Size4KiB>::new(3);
        let f = format!("{:?}", n);

        assert_eq!(format!("NumOfPages::<Size4KiB>(3)"), f);
    }
}
