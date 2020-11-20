# `erfa-sys`

This crate provides direct bindings to the
[ERFA](https://github.com/liberfa/erfa) C library. It is possible to build the
library from C sources, such that the user's system does not need the ERFA C
library available as a prerequisite - see static linking below.

## Static linking
This crate supports linking to a static build of ERFA. The advantage of doing
this is that you don't need the ERFA library available as a system library at
runtime.

Static linking can be done in two ways. The first is by using the `static`
feature when building `erfa-sys`; this compiles `erfa-src` using your own C
compiler.

The second way to link ERFA statically is by providing a `liberfa.a` library.
Either the ERFA_STATIC or PKG_CONFIG_ALL_STATIC environment variable need to be
set to 1. The directory containing this file (or the `liberfa.so` file) can be
specified with `ERFA_LIB`. If `ERFA_LIB` isn't found, then `pkg-config` is used
to search for the library.

## Updating the provided ERFA source code
The `erfa-sys/ext` directory contains the contents of an ERFA release tarball
provided by [GitHub](https://github.com/liberfa/erfa/releases). If the source
code needs to be changed, then the existing contents should be replaced with
another tarball's contents. If the library version has changed, then attempting
to build `erfa-sys` (with e.g. `cargo build`) should fail; the directory name
will need to be changed in `erfa-sys/build.rs` (the error message from the build
is right next to what needs to be changed).

If new ERFA functions or constants are introduced, then `gen_rust_include.sh`
should also be run.

## MacOS and/or Windows
I don't have access to these systems, so I don't know how they would work here.
Pull requests welcome.

## Acknowledgement
This crate was made with a lot of help and inspiration from the
[`hdf5-rust`](https://github.com/aldanor/hdf5-rust) crate family.
