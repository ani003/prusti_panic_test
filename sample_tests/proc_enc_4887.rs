// thread 'rustc' panicked at 'not implemented: Only ZSTs are currently supported, got: [i32, i32]'
// prusti-viper/src/encoder/procedure_encoder.rs:4887:25

const C: (i32, i32) = (0, 0);

fn main() {
    let _ = C.0;
}