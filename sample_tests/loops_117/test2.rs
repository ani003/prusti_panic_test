// thread 'rustc' panicked at 'not implemented: NonMutatingUse(ShallowBorrow)'
// prusti-interface/src/environment/loops.rs:117:22

fn main() {
    loop {
        match Some(1) {
            Some(c) if c <= 1 => (),
            Some(_) => (),
            None => (),
        }
    }
}
