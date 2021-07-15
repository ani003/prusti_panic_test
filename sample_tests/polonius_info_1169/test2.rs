// thread 'rustc' panicked at 'no entry found for key'
// prusti-interface/src/environment/polonius_info.rs:1169:9

fn foo(ptr: *mut i32) {
    let _ = ptr as *mut i32;
}

fn main() {}
