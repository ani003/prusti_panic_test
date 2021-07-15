// thread 'rustc' panicked at 'assertion failed: !state.contains_pred(&prefix)'
// prusti-viper/src/encoder/foldunfold/semantics.rs:92:25

fn foo() -> *const i32 { std::ptr::null() }

fn main() {
    let _ : &'static i32 = unsafe { &*foo() };
}
