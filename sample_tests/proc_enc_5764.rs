// thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1'
// prusti-viper/src/encoder/procedure_encoder.rs:5764:36

union Foo {
    a: [i32; 1],
    b: [i32; 1],
}

fn main() {
    let _ = Foo { a: [0] };
}