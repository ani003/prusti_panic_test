// thread 'rustc' panicked at 'not implemented: NonMutatingUse(AddressOf)'
// prusti-interface/src/environment/loops.rs:117:22

const Foo: &'static i32 = &0;

fn bar() {
    loop {
        let _ = Foo as *const i32;
    }
}

fn main() {}
