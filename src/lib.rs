//! This crate provides a data structure for byte size. With the
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
#![cfg_attr(not(test), no_std)]
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

mod bytes;
mod num_of_pages;

pub use bytes::Bytes;
pub use num_of_pages::NumOfPages;
