// thread 'rustc' panicked at 'internal error: entered unreachable code: __t4 := builtin$havoc_ref()'
// prusti-viper/src/encoder/foldunfold/mod.rs:455:26

fn bar(x: &[u64; 1]) -> &u64 {
    &x[0]
}

fn main() {}
