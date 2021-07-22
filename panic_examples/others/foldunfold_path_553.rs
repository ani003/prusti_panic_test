// thread 'rustc' panicked at 'perm_amount is write, but it should be >= write-read'
// prusti-viper/src/encoder/foldunfold/path_ctxt.rs:553:13

#[derive(Clone)]   // Commenting this line, makes the panic go away
pub enum Token<'a, O> {     // All variants are required
    RBrace,
    Operator(O),
    Variable(&'a [i32]),
    Malformed(&'a [i32]),
}

fn main() {}