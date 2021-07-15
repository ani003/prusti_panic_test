// thread 'rustc' panicked at 'internal error: entered unreachable code: __t4 := builtin$havoc_ref()'
// prusti-viper/src/encoder/foldunfold/mod.rs:453:26

struct Foo([u64; 1]);

fn bar(x: &Foo) -> &u64 {
    &x.0[0]
}

fn main() {}
