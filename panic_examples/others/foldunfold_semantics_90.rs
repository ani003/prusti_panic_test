// thread 'rustc' panicked at 'assertion failed: !state.contains_pred(&prefix)'
// prusti-viper/src/encoder/foldunfold/semantics.rs:90:25

fn main() {
    let _ : &'static i32 = unsafe { &*std::ptr::null() };
}
