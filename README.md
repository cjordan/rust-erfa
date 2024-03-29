# `rust-erfa`

<img src="https://img.shields.io/github/workflow/status/cjordan/rust-erfa/Cross-platform%20tests?label=Cross-platform%20tests&logo=github" alt="Cross-platform%20tests">

This project contains the crates `erfa` and `erfa-sys`, which allow usage of the
[`ERFA`](https://github.com/liberfa/erfa) library in Rust.

# `erfa`

<a href="https://crates.io/crates/erfa">
  <img src="https://img.shields.io/crates/v/erfa?logo=rust" alt="crates.io"></a>
<a href="https://docs.rs/crate/erfa">
  <img src="https://img.shields.io/docsrs/erfa?logo=rust" alt="docs.rs"></a>

This crate is a pure-Rust equivalent to the
[`ERFA`](https://github.com/liberfa/erfa) C library. The C library is relatively
easy to re-write as it has no dependencies, and it is convenient to have a
Rust-re-written library to avoid complications during C library installation.

However, it is currently incomplete. I've implemented many functions but only
the ones I need. Please file a PR or issue if you require more.

This library is tested against `erfa-sys`, effectively meaning that the results
are the same as the original C library.

# `erfa-sys`

<a href="https://crates.io/crates/erfa-sys">
  <img src="https://img.shields.io/crates/v/erfa-sys?logo=rust" alt="crates.io"></a>
<a href="https://docs.rs/crate/erfa-sys">
  <img src="https://img.shields.io/docsrs/erfa-sys?logo=rust" alt="docs.rs"></a>

This crate provides direct bindings to the
[ERFA](https://github.com/liberfa/erfa) C library. It is possible to build the
library from C sources, such that the user's system does not need the ERFA C
library available as a prerequisite - see static linking below.

## Static linking
This crate supports linking to a static build of ERFA. The advantage of doing
this is that you don't need the ERFA library available as a system library at
runtime.

Static linking can be done in two ways. The first is by using the `static`
feature when building `erfa-sys`; this compiles the bundled ERFA source code
using your own C compiler.

The second way to link ERFA statically is by providing a `liberfa.a` library.
Either the `ERFA_STATIC` or `PKG_CONFIG_ALL_STATIC` environment variable need to
be set to 1. The directory containing this file (or the `liberfa.so` file) can
be specified with `ERFA_LIB`. If `ERFA_LIB` isn't found, then `pkg-config` is
used to search for the library.

## Updating the provided ERFA source code
The `ext` directory contains a git submodule of the [ERFA
repo](https://github.com/liberfa/erfa). If the source code needs to be changed,
then the git submodule can use a different commit.

If new ERFA functions or constants are introduced, then `gen_rust_include.sh`
should also be run.

## Windows
I don't know how to run/bind to ERFA on Windows, so this crate may not work
there. Pull requests welcome.

## Acknowledgement
This crate was made with a lot of help and inspiration from the
[`hdf5-rust`](https://github.com/aldanor/hdf5-rust) crate family.
