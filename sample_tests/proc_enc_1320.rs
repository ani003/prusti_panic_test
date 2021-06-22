// thread 'rustc' panicked at 'not implemented: encoding of 'move _2 as *const i32 (Pointer(MutToConstPointer))''
// prusti-viper/src/encoder/procedure_encoder.rs:1320:17

fn main() {
    let _: *const i32 = std::ptr::null_mut();
}