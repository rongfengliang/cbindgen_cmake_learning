from cffi import FFI

ffi = FFI()

with open("bindings.h") as f:
    ffi.cdef(f.read())


lib = ffi.dlopen("target/release/libmylib.dylib")

print(lib.addv2(1, 2))

