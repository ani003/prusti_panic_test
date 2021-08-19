#[derive(Clone)]     // Commenting this line, makes the panic go away
enum Token<'a> {     // All variants are required
    RBrace,
    Operator,
    Variable(&'a i32),
    Malformed(&'a i32),
}

fn main() {}

/*
thread 'rustc' panicked at 'perm_amount is write, but it should be >= write-read', prusti-viper/src/encoder/foldunfold/path_ctxt.rs:553:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: std::panicking::begin_panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:460:5
   2: prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt::obtain
   3: prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt::join::{{closure}}
   4: prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt::join
   5: <prusti_viper::encoder::foldunfold::FoldUnfold as vir::legacy::cfg::visitor::CfgReplacer<prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt,prusti_viper::encoder::foldunfold::ActionVec>>::prepend_join
   6: prusti_viper::encoder::foldunfold::add_fold_unfold
   7: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode
   8: prusti_viper::encoder::encoder::Encoder::encode_procedure
   9: prusti_viper::encoder::encoder::Encoder::process_encoding_queue
  10: prusti_viper::verifier::Verifier::verify
  11: prusti_driver::verifier::verify
  12: <prusti_driver::callbacks::PrustiCompilerCalls as rustc_driver::Callbacks>::after_analysis
  13: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  14: rustc_span::with_source_map
  15: rustc_interface::interface::create_compiler_and_run
  16: scoped_tls::ScopedKey<T>::set
*/
