// thread 'rustc' panicked at 'key error: __TYPARAM__$T$__'
// prusti-common/src/vir/ast/typaram.rs:95:79

fn foo<T: Copy>(values: &[T], ixs: &[usize]) {
    let _ = ixs.iter().map(|_| values[0]);
}

fn main() {}