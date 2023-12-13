local ffi = require('ffi')

local C = ffi.C

lib  = ffi.load('./target/release/libmylib.dylib', true)

ffi.cdef[[
    int32_t addv2(int32_t a, int32_t b);
    typedef struct Foo {
        int32_t a;
        int32_t b;
        char *c;
    } Foo;
    bool init_app(struct Foo foo);
]]

local result = lib.addv2(1, 2)

print(result)

local myFooInstance = ffi.new("Foo")

myFooInstance.a = 1
myFooInstance.b = 2

local text = "dalongdemo"
print(myFooInstance.c)

local c_str = ffi.new("char[?]", #text + 1)

ffi.copy(c_str,text)

print(c_str)

myFooInstance.c = c_str
print(myFooInstance.c)
local  check_result = lib.init_app(myFooInstance)

print(check_result)


