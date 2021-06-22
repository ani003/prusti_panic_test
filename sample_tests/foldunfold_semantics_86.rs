// thread 'rustc' panicked at 'The rhs place of statement '_5.val_ref := borrow _1.val_ref // L1' is currently moved-out or blocked due to a borrow'
// prusti-viper/src/encoder/foldunfold/semantics.rs:86:21

fn foo(a: &mut [i32], l: usize) {}
 
fn bar(a: &mut [i32]) {
    foo(a, a.len());    // Requires both
}
 
fn main() {}