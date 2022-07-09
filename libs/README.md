# Libraries

This folder contains the libraries used by `tkern`.

# Guidelines

## Creating a new crate

When creating a new crate, the following should be kept in mind:
- Make sure the crate name doesn't already exist on crates.io, even if it's never going to be published. It messes with RLS. If a crate name would conflict, please prefix it with `tkern_`.
- The cargo.toml should have:
  - `version = "0.0.0"` (unless it is published on crates.io)
  - `edition = "2021"`
  - `license = "MIT OR Apache-2.0"`
- The `src/lib.rs` should have:
  - `#![cfg_attr(not(test), no_std)]`
  - `#![feature(strict_provenance)]` or `#![cfg_attr(feature = "nightly", feature(strict_provenance))]`
  - `#![warn(unsafe_op_in_unsafe_fn, clippy::pedantic, rust_2018_idioms)]`
- The `src/lib.rs` may allow these lints:
  - `#![allow(clippy::cast_possible_truncation, clippy::used_underscore_binding)]`
