use std::ptr;

fn main() {
    let p: *const i32 = ptr::null();
    assert!(p.is_null());
}