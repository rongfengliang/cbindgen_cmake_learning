use libc::{c_char, c_int, c_void};

use std::ffi::CStr;
#[repr(C)]
struct  Foo {
    a:i32,
    b:i32,
    c: *mut  c_char
}


#[no_mangle]
pub extern  "C"  fn sub(a:i32,b:i32) -> i32 {
    a - b
}

#[no_mangle]
pub extern  "C" fn  addv2(a:i32,b:i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern  "C" fn init_app(foo: Foo) -> bool {
    if foo.c.is_null() {
        return false
    } else{
        let c_str = unsafe { CStr::from_ptr(foo.c) };
        println!("foo.c:{:?}", c_str.to_string_lossy().into_owned());
        return true
    }
}