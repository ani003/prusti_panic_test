// thread 'rustc' panicked at 'not implemented: Param(impl FnMut(usize) -> i32/#0)'
// prusti-viper/src/encoder/mir_encoder/mod.rs:198:30

fn foo(mut bar: Vec<i32>, mut f: impl FnMut(usize) -> i32){
    let _ = bar.iter_mut().enumerate().for_each(move |(_, edge)| *edge = f(0));     //Requires enumerate
}

fn main() {}