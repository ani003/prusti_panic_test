// thread 'rustc' panicked at 'no entry found for key'
// prusti-interface/src/environment/polonius_info.rs:1169:9

#[derive(Clone)]
struct Foo<'a> {
    x: &'a [u8],
}

fn bar(foo: &Foo) {
    let _ = Foo {
        ..foo.clone()
    };
}

fn main() {}
