// thread 'rustc' panicked at 'not implemented: Param(T/#0)'
// prusti-viper/src/encoder/type_encoder.rs:146:22

fn foo<T: Ord> (bar: &mut [T]) {
    let _ = bar[0] == bar[0];
}

fn main() {}