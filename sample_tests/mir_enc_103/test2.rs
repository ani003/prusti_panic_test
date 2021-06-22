// thread 'rustc' panicked at 'Type &mut impl FnMut(usize) -> i32 has no fields'
// prusti-viper/src/encoder/mir_encoder/mod.rs:103:25

fn foo(mut bar: Vec<i32>, mut f: impl FnMut(usize) -> i32){
    let _ = bar.iter_mut().enumerate().for_each(|(_, edge)| *edge = f(0));
}

fn main() {}