// thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value'
// prusti-viper/src/encoder/procedure_encoder.rs:2564:57

fn main() {
    let x = |_, _| 0;
    let _ = x(0, 0);
}