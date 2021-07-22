// thread 'rustc' panicked at 'Box<dyn Any>'
// /rustc/ed597e7e19d0fe716d9f81b1e840a5abbfd7c28d/compiler/rustc_errors/src/lib.rs:953:9

struct Foo;

struct Bar(Foo);

fn foo(r: Option<Foo>) {
    let _ = r.map(Bar);
}

fn main() {}
