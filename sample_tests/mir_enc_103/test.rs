// thread 'rustc' panicked at 'Type usize has no fields'
// prusti-viper/src/encoder/mir_encoder/mod.rs:103:25

fn foo(bar: Vec<Vec<u32>>) {
    let _ = bar.iter().enumerate().flat_map(|(y, r)| r.iter().enumerate().map(move |_| y));
}
 
fn main() {}