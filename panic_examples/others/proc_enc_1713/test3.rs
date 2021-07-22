// thread 'rustc' panicked at 'assertion failed: `(left == right)`
//  left: `0`,
//  right: `1`: We can have at most one magic wand in the postcondition.'
// prusti-viper/src/encoder/procedure_encoder.rs:1713:9

fn foo(x: &i32) -> &'static i32 { &0 }  // Arg must be ref; Requires lifetime

fn bar(s: &'static i32) {}

fn abc(x: &i32) {
    bar(foo(x));
}

fn main() {}
