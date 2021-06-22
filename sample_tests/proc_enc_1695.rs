// thread 'rustc' panicked at 'assertion failed: `(left == right)`
//  left: `0`,
//  right: `1`: We can have at most one magic wand in the postcondition.'
// prusti-viper/src/encoder/procedure_encoder.rs:1695:9 

fn foo(mut bar: Vec<i32>) {
    let _ = bar.iter().nth(0).unwrap();
}
 
fn main() {}