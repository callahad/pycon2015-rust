#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from cffi import FFI
ffi = FFI()

ffi.cdef("""
    long triple(long long x);
""")

C = ffi.dlopen("target/debug/libburn-ea11bffea0950ae2.dylib")

print(C.triple(3))
