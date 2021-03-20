# Changelog

## Unreleased - ReleaseDate
## Added
- `Bytes` and `NumOfPages` now implements `Add<usize>`, `Sub<usize>`, `SubAssign<usize>`, `From<usize>`, and `Hash`.

### Changed
- `#[must_use]` attribute is added to some methods.

### Removed
- The implementations of `Mul` and `MulAssign` for the product of `Bytes` and `Bytes`, and `NumOfPages` and `NumOfPages` are removed. The units of these products are either the square of `Bytes` or `NumOfPages`, and not just `Bytes` or `NumOfPages`.

## 0.2.7 - 2020-12-29

- Update the version of `x86_64` crate to 0.13.0

## 0.2.6 - 2020-12-28

- Implement `zero` method.

## 0.2.5 - 2020-10-30

- Implement `AddAssign` with `usize`

## 0.2.4 - 2020-10-25

- Fix doc.
- Fix README.

## 0.2.3 - 2020-10-24

- Implement `DivAssign` trait.

## 0.2.2 - 2020-10-24

- Implement `Div` trait.

## 0.2.1 - 2020-10-24

- Implement `MulAssign` trait.

## 0.2.0 - 2020-10-24

- Split `Size` into `Bytes` and `NumOfPages`

## 0.1.4 - 2020-10-24

- Implement `Mul` trait.

## 0.1.3 - 2020-09-30

- Fix a build error on latest nightly.

## 0.1.2 - 2020-09-24

- Update the version of `x86_64` crate to 0.12.0

## 0.1.1 - 2020-09-07

- Implement `AddAssign` and `SubAssign` traits.

## 0.1.0 - 2020-09-06

- Initial release.
