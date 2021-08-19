fn main() {
    let x = |_, _| 0;
    let _ = x(0, 0);
}

/*
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', prusti-viper/src/encoder/procedure_encoder.rs:2582:57
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: core::panicking::panic
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:50:5
   3: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_impure_function_call
   4: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_terminator
   5: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_statement_at
   6: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_blocks_group
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
