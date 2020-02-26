# Rust Bindings to Memory Module Library

This repository contains low-level Rust bindings to the
library at https://github.com/fancycode/MemoryModule that facilitates
loading Windows PE files from memory on Windows. i.e. it allows you to
import a DLL/EXE from a `&[u8]` without requiring the library to be
backed by a file.

The Rust library simply compiles the upstream library and declares
`extern "C"` unsafe bindings to it. This being a `-sys` crate, that
is all it will ever do: higher-level bindings would exist in a separate
crate.

# License

This crate is licensed under the same terms as the `MemoryModule` library,
which is `MPL 2.0`.

The `MemoryModule` library is maintained by Joachim Bauch.
