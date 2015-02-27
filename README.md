# dueboot

Based on https://github.com/neykov/armboot, and is a template for Arduino Due projects.

## Compiling

Modify the `Makefile` with the following information:
- Location of your Arduino installation 
- Location of your `rustc` executable
- Location of the `llc` used by `rustc`. NOTE: Using external LLVM = wierdness
- Location of the source of Rust's `libcore`
- Source file for your primary crate

You can then run `make all` to build the binaries, or `make flash` to flash them
to the arduino.

## Editing

Some example code can be found in `main.rs`. Edit this file to customize your
code.

## Notes

As this program does not link against Rust's `libstd` (for obvious reasons). You
have to link against `libcore` manually to get some of the tools necessary for
writing idiomatic Rust (like `Option<..>` and `Result<..>` types). The linking
is handled manually by the Makefile, but you must still tell Rust to look for
the metadata for `libcore` by including the following in your crates:

````rust
extern crate core;
````

`libstd` also re-exports a lot of `libcore` by default, so you'll notice that
you will have to do something like the following quite often to improve quality
of life for those used to a more normal Rust development environment.

````rust
use core::option::Option;
use core::option::Option::{None, Some};
````

## Credits

  - armboot: https://github.com/neykov/armboot
