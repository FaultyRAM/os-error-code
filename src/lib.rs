// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Functions for obtaining and updating the platform-specfic last error code.
//!
//! This crate provides cross-platform functionality for obtaining and modifying the
//! platform-specific last error code (e.g. `errno` on Unices). The `get_last_error` and
//! `set_last_error` functions can be used to read and write the last error code, respectively.
//!
//! # Example
//!
//! ```rust
//! extern crate os_error_code;
//!
//! fn main() {
//!     os_error_code::set_last_error(1);
//!     assert_eq!(os_error_code::get_last_error(), 1);
//! }
//! ```

#![no_std]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy_internal))]
#![cfg_attr(feature = "clippy", deny(clippy_pedantic))]
#![cfg_attr(feature = "clippy", forbid(clippy_restrictions))]
#![forbid(warnings)]
#![forbid(anonymous_parameters)]
#![forbid(box_pointers)]
#![forbid(fat_ptr_transmutes)]
#![forbid(missing_copy_implementations)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(trivial_casts)]
#![forbid(trivial_numeric_casts)]
#![forbid(unused_extern_crates)]
#![forbid(unused_import_braces)]
#![forbid(unused_qualifications)]
#![forbid(unused_results)]
#![forbid(variant_size_differences)]

#[cfg(target_os = "dragonfly")]
extern crate dragonfly_errno;
#[cfg(all(unix, not(target_os = "dragonfly")))]
extern crate libc;
#[cfg(windows)]
extern crate winapi;

#[cfg(unix)]
#[path = "unix.rs"]
mod sys;

#[cfg(windows)]
#[path = "windows.rs"]
mod sys;

#[inline]
/// Returns the platform-specific last error code.
pub fn get_last_error() -> i32 {
    sys::get_last_error()
}

#[inline]
/// Sets the platform-specific last error code to the given value.
pub fn set_last_error(code: i32) {
    sys::set_last_error(code)
}
