// thread 'rustc' panicked at 'not implemented: MutatingUse(AddressOf)'
// prusti-interface/src/environment/loops.rs:117:22

fn main() {
    let mut x: i32 = 0;
    loop {
        unsafe { &mut x as *mut i32; }
    }
}