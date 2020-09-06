# os_units

[![Crates.io](https://img.shields.io/crates/v/os_units)](https://crates.io/crates/os_units)
[![docs.rs](https://docs.rs/os_units/badge.svg)](https://docs.rs/os_units)
[![Rust](https://github.com/toku-sa-n/os_units/workflows/Rust/badge.svg)](https://github.com/toku-sa-n/os_units/actions)

This crate provides a data structure for byte size. With
[`x86_64`](https://github.com/rust-osdev/x86_64) crate, you can easily convert
the size of physical memory pages into bytes, and bytes into the number of physical memory
pages.

## Examples

```rust
use os_units::{Size, Bytes};
use x86_64::structures::paging::{PageSize, Size4KiB};

let bytes_of_kernel = Size::<Bytes>::new(314159);
let pages_of_kernel = bytes_of_kernel.as_num_of_pages::<Size4KiB>();
assert_eq!(pages_of_kernel.as_usize(), 77);

let bytes_of_pages = pages_of_kernel.as_bytes();
assert_eq!(bytes_of_pages.as_usize(), 315392);
```

License: MPL-2.0
