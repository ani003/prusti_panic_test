fn main() {
    let _ : &'static i32 = unsafe { &*std::ptr::null() };
}

/*
thread 'rustc' panicked at 'assertion failed: !state.contains_pred(&prefix)', prusti-viper/src/encoder/foldunfold/semantics.rs:90:25
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: core::panicking::panic
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:50:5
   3: <vir::legacy::ast::stmt::Stmt as prusti_viper::encoder::foldunfold::semantics::ApplyOnState>::apply_on_state
   4: <prusti_viper::encoder::foldunfold::FoldUnfold as vir::legacy::cfg::visitor::CfgReplacer<prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt,prusti_viper::encoder::foldunfold::ActionVec>>::replace_stmt
   5: prusti_viper::encoder::foldunfold::add_fold_unfold
   6: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode
   7: prusti_viper::encoder::encoder::Encoder::encode_procedure
   8: prusti_viper::encoder::encoder::Encoder::process_encoding_queue
   9: prusti_viper::verifier::Verifier::verify
  10: prusti_driver::verifier::verify
  11: <prusti_driver::callbacks::PrustiCompilerCalls as rustc_driver::Callbacks>::after_analysis
  12: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  13: rustc_span::with_source_map
  14: rustc_interface::interface::create_compiler_and_run
  15: scoped_tls::ScopedKey<T>::set
*/
