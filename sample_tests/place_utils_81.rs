// thread 'rustc' panicked at 'not implemented: ty=Closure(DefId(0:4 ~ place_utils_81[317d]::main::{closure#0}), [i16, extern "rust-call" fn((i32,)), (&mut std::vec::Vec<i32>,)])'
// analysis/src/abstract_domains/place_utils.rs:81:17

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let _ = (0..1).map(|_| {
        v = Vec::new();
    });
}