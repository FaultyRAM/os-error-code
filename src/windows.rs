// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Windows-specific implementation details.

use winapi::um::errhandlingapi;
use winapi::shared::minwindef::DWORD;

#[cfg_attr(feature = "clippy", allow(cast_possible_wrap))]
#[inline]
/// Windows-specific `get_last_error` implementation.
pub fn get_last_error() -> i32 {
    unsafe { errhandlingapi::GetLastError() as i32 }
}

#[cfg_attr(feature = "clippy", allow(cast_sign_loss))]
#[inline]
/// Windows-specific `set_last_error` implementation.
pub fn set_last_error(code: i32) {
    unsafe { errhandlingapi::SetLastError(code as DWORD) }
}
