Super rough examples of calling Rust functions from Python via FFI.

Everything uses Python 3, the "cffi" (`pip install cffi`) library, and rustc 1.0.0-nightly (12b846ab8 2015-03-09).

After running `cargo build` be sure to update the call to ffi.dlopen in the python files.
