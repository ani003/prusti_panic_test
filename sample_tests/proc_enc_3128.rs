// thread 'rustc' panicked at 'assertion failed: `(left == right)`
//  left: `2`,
//  right: `1`: We can have at most one magic wand in the postcondition.'
// prusti-viper/src/encoder/procedure_encoder.rs:3128:13

struct Foo<'a> {    // Requires both with different lifetimes
    x: &'static str,
    y: &'a str,
}

impl<'a> Foo<'a> {
    fn new(x: &'static str, y: &'a str) -> Self {
        Foo { x, y }
    }
}

fn bar<'a>(x: &'static str, y: &'a str) {   // Required
    let _ = Foo::new(x, y);
}

fn main() {}
