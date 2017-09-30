// Copyright (c) 2017 FaultyRAM
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

//! Unix-specific implementation details.

#[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "fuchsia",
            target_os = "l4re"))]
use libc::__errno_location as errno_location;
#[cfg(any(target_os = "bitrig", target_os = "netbsd", target_os = "openbsd",
            target_os = "android", target_env = "newlib"))]
use libc::__errno as errno_location;
#[cfg(target_os = "solaris")]
use libc::___errno as errno_location;
#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
use libc::__error as errno_location;
#[cfg(target_os = "haiku")]
use libc::_errnop as errno_location;
#[cfg(target_os = "dragonfly")]
use dragonfly_errno::errno_location;

#[inline]
/// Unix-specific `get_last_error` implementation.
pub fn get_last_error() -> i32 {
    unsafe { *errno_location() }
}

#[inline]
/// Unix-specific `set_last_error` implementation.
pub fn set_last_error(code: i32) {
    unsafe {
        *errno_location() = code;
    }
}
