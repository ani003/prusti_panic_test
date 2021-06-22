// thread 'rustc' panicked at 'assertion failed: perm_amount.is_valid_for_specs()'
// prusti-viper/src/encoder/foldunfold/mod.rs:890:21

fn main() {
    let n = [0];
    let _ = &n[0..];
}