// thread 'rustc' panicked at 'no entry found for key'
// prusti-interface/src/environment/polonius_info.rs:1169:9

fn foo(p: *const i32) {
    let _ = p.is_null();
}

fn main() {}