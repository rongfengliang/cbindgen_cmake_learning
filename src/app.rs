#[repr(C)]
pub struct  Foo {
    a:i32,
    b:i32,
    c: *mut std::os::raw::c_char
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
        return true
    }
}