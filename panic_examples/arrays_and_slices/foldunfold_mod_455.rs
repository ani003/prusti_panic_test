fn bar(x: &[u64; 1]) -> &u64 {
    &x[0]
}

fn main() {}

/*
thread 'rustc' panicked at 'internal error: entered unreachable code: __t4 := builtin$havoc_ref()', prusti-viper/src/encoder/foldunfold/mod.rs:455:26
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
   3: prusti_viper::encoder::foldunfold::process_expire_borrows::<impl prusti_viper::encoder::foldunfold::FoldUnfold>::process_expire_borrows
   4: <prusti_viper::encoder::foldunfold::FoldUnfold as vir::legacy::cfg::visitor::CfgReplacer<prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt,prusti_viper::encoder::foldunfold::ActionVec>>::replace_stmt
   5: <prusti_viper::encoder::foldunfold::FoldUnfold as vir::legacy::cfg::visitor::CfgReplacer<prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt,prusti_viper::encoder::foldunfold::ActionVec>>::replace_stmt
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
