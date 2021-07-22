// thread 'rustc' panicked at 'internal error: entered unreachable code: &/*tls*/ FOO::__getit::__KEY'
// prusti-interface/src/environment/polonius_info.rs:613:22

use std::cell::Cell;

type Bar = fn(&dyn FnMut()) -> u32;

fn foo(f: &dyn FnMut()) -> u32 { 1 }

thread_local! {
    static FOO: Cell<Bar> = Cell::new(foo);
}

fn main() {}