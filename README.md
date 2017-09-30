# os-error-code

[![Travis](https://img.shields.io/travis/FaultyRAM/os-error-code.svg)][1]
[![AppVeyor](https://img.shields.io/appveyor/ci/FaultyRAM/os-error-code.svg)][2]
[![Crates.io](https://img.shields.io/crates/v/os-error-code.svg)][3]
[![Docs.rs](https://docs.rs/os-error-code/badge.svg)][4]

This crate provides cross-platform functionality for obtaining and modifying the platform-specific
last error code (e.g. `errno` on Unices). Because it is `no_std`-friendly, it can be used in
low-level code that directly interfaces with C, and as a building block for higher-level
abstractions of platform-specific error handling.

## Example

```rust
extern crate os_error_code;

fn main() {
    os_error_code::set_last_error(1);
    assert_eq!(os_error_code::get_last_error(), 1);
}
```

## License

Licensed under either of

* Apache License, Version 2.0,
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[1]: https://travis-ci.org/FaultyRAM/os-error-code
[2]: https://ci.appveyor.com/project/FaultyRAM/os-error-code
[3]: https://crates.io/crates/os-error-code
[4]: https://docs.rs/os-error-code
