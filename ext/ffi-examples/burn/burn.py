#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from cffi import FFI
ffi = FFI()

ffi.cdef("""
    int triple(int x);
""")

C = ffi.dlopen("target/debug/libburn-c21ba537b4f26cae.dylib")

print(C.triple(3))
