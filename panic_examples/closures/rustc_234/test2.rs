n foo(x: &Option<usize>) {
    let _ = x.and_then(usize::checked_next_power_of_two);
}

fn main() {}

/*
thread 'rustc' panicked at 'DefId::expect_local: `DefId(2:25741 ~ core[734c]::num::{impl#11}::checked_next_power_of_two)` isn't local', /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/compiler/rustc_span/src/def_id.rs:234:43
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: std::panicking::begin_panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:460:5
   2: rustc_span::def_id::DefId::expect_local::{{closure}}
   3: prusti_interface::environment::procedure::Procedure::new
   4: prusti_interface::environment::Environment::get_procedure
   5: prusti_viper::encoder::encoder::Encoder::encode_spec_funcs
   6: prusti_viper::encoder::encoder::Encoder::encode_const_expr
   7: prusti_viper::encoder::mir_encoder::MirEncoder::encode_operand_expr
   8: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_impure_function_call
   9: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_terminator
  10: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_statement_at
  11: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_blocks_group
  12: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode
  13: prusti_viper::encoder::encoder::Encoder::encode_procedure
  14: prusti_viper::encoder::encoder::Encoder::process_encoding_queue
  15: prusti_viper::verifier::Verifier::verify
  16: prusti_driver::verifier::verify
  17: <prusti_driver::callbacks::PrustiCompilerCalls as rustc_driver::Callbacks>::after_analysis
  18: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  19: rustc_span::with_source_map
  20: rustc_interface::interface::create_compiler_and_run
  21: scoped_tls::ScopedKey<T>::set
*/
