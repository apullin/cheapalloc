# `cheapalloc`

A simple crate to provide a [GlobalAllocator](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/global-allocators.html) that uses libc malloc & free, 
thus using an existing C heap. (thus the name: "C heap alloc")

The intended use is to enable heap-dependent features in Rust, e.g. Box, alloc::vec::Vec, alloc::string::String, etc when using 
Rust libraries via FFI from C, such that they can use the existing C heap. 

The original mission of this crate was to enable use of Rust to make staticlibs for embedded applications on microcontrollers.

## Dependencies

Requires nightly compiler due to the use of `#![feature(lang_items)]`.

## Using this Crate

Add the crate as a dependency, and the global allocator should be provided. \
You will need to amend your linking system to 

TODO: Linker instructions, e.g. `--print-sysroot`, `-Wl,-lc`, etc

**NOTE**: This crate can optionally define a out-of-memory `panic_halt` / `oom` handler via the `provide_oom` feature.
This is currently implemented specifically for the Cortex-M architecture, with the idea that it alleviates the
user/library from needing to include this block themselves.

## Example

TODO: Include an example, or provide one in a separate github repo. Seriously powerful stuff like serde can be made to
work on Cortex-M micros!

## Pitfalls

Beyond the scope of this crate, but: linking a rust staticlib with an existing C program does present some issues
due to the use of `compiler-builtins` by rust.

Some discussion [here](https://github.com/rust-lang/compiler-builtins/issues/345).

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
