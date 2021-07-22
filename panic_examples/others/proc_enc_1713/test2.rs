// thread 'rustc' panicked at 'assertion failed: `(left == right)`
//  left: `0`,
//  right: `1`: We can have at most one magic wand in the postcondition.'
// prusti-viper/src/encoder/procedure_encoder.rs:1713:9

fn main() {
   unsafe { let _: &usize = std::mem::transmute(1usize); }
}
