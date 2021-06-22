// thread 'rustc' panicked at 'Type [closure@saved_tests/mir_enc_261_strange.rs:5:25: 5:33] can not be dereferenced'
// prusti-viper/src/encoder/mir_encoder/mod.rs:261:9

fn main() {
    let v = Some(0).map(|_| &[1]);
}