// thread 'rustc' panicked at 'not implemented: ty=Closure(DefId(0:5 ~ prusti_interface_src_81[317d]::main::{closure#0}), [i16, extern "rust-call" fn((i32,)), (&u32,)])'
// prusti-interface/src/utils.rs:81:17

fn foo(_: u32) {}
 
fn main() {
    let n = 0;
    let _ = (0..1).map(|_| {
        loop {
            foo(n);
        }
    });
}