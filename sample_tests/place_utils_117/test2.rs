// thread 'rustc' panicked at 'not implemented: elem = Index(_8)'
// analysis/src/abstract_domains/place_utils.rs:117:13

enum Foo {
    A(i32),
    B(String),  // Requires this and must be string
}

fn main() {
    let mut bar = [
        Foo::A(0),
    ];

    match bar[0] {
        Foo::A(_) => bar[0] = Foo::A(0),
        _ => {},
    }
}
