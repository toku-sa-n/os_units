# Changelog

## Unreleased - ReleaseDate
### Removed
- `#![deny(warnings)]` is removed from the source code.

## 0.4.0 - 2021-04-28
### Changed
- Some methods lost the `const` attribute: `Bytes::as_usize` and all methods of `NumOfPages`.
- The stable Rust users also can use this library now.

## 0.3.1 - 2021-04-12
### Changed
- Update the version of `x86_64` crate to 0.14.0

## 0.3.0 - 2021-03-20
### Added
- `Bytes` and `NumOfPages` now implements `Add<usize>`, `Sub<usize>`, `SubAssign<usize>`, `From<usize>`, `Display`, and `Hash`.
- The debug print of the `NumOfPages` type is improved.

### Changed
- **LICENSE is changed from Mozilla Public License 2.0 to "MIT License OR Apache License 2.0"**.
- `#[must_use]` attribute is added to some methods.

### Removed
- The implementations of `Mul` and `MulAssign` for the product of `Bytes` * `Bytes`, and `NumOfPages` * `NumOfPages` are removed. The units of these products are either the square of `Bytes` or `NumOfPages`, and not just `Bytes` or `NumOfPages`.

## 0.2.7 - 2020-12-29
### Changed
- Update the version of `x86_64` crate to 0.13.0

## 0.2.6 - 2020-12-28
### Added
- Implement `zero` method.

## 0.2.5 - 2020-10-30
### Added
- Implement `AddAssign` with `usize`

## 0.2.4 - 2020-10-25
### Fixed
- Fix doc.
- Fix README.

## 0.2.3 - 2020-10-24
### Added
- Implement `DivAssign` trait.

## 0.2.2 - 2020-10-24
### Added
- Implement `Div` trait.

## 0.2.1 - 2020-10-24
### Added
- Implement `MulAssign` trait.

## 0.2.0 - 2020-10-24
### Changed
- Split `Size` into `Bytes` and `NumOfPages`

## 0.1.4 - 2020-10-24
### Added
- Implement `Mul` trait.

## 0.1.3 - 2020-09-30
### Fixed
- Fix a build error on latest nightly.

## 0.1.2 - 2020-09-24
### Changed
- Update the version of `x86_64` crate to 0.12.0

## 0.1.1 - 2020-09-07
### Added
- Implement `AddAssign` and `SubAssign` traits.

## 0.1.0 - 2020-09-06

- Initial release.
