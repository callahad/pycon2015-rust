#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from cffi import FFI

ffi = FFI()
ffi.cdef("""
    long long calculate(const char *script);
""")
C = ffi.dlopen("target/release/libcalc-35d9862849daffe3.dylib")

def main():
    result = C.calculate(b"+ * * + * - /")
    print(result)

if __name__ == '__main__':
    main()

