// thread 'rustc' panicked at 'Box<dyn Any>'
// /rustc/ed597e7e19d0fe716d9f81b1e840a5abbfd7c28d/compiler/rustc_errors/src/lib.rs:953:9

fn foo(f: impl Fn(usize)) {}

trait Bar {
    fn run(self);
}
 
impl Bar for usize {
    fn run(self) {}
}
 
fn main() {
    foo(Bar::run);
}