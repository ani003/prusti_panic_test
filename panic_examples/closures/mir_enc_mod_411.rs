struct Foo;

impl Foo {
    fn bar(&self) {
        let abc = |x: &Foo| ();
        abc(self);
    }
}

fn main() {}

/*
thread 'rustc' panicked at 'index out of bounds: the len is 7 but the index is 7', prusti-viper/src/encoder/mir_encoder/mod.rs:411:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: core::panicking::panic_bounds_check
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:69:5
   3: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_local_variable_permission
   4: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_precondition_expr
   5: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_impure_function_call
   6: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_terminator
   7: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_statement_at
   8: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_blocks_group
   9: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode
  10: prusti_viper::encoder::encoder::Encoder::encode_procedure
  11: prusti_viper::encoder::encoder::Encoder::process_encoding_queue
  12: prusti_viper::verifier::Verifier::verify
  13: prusti_driver::verifier::verify
  14: <prusti_driver::callbacks::PrustiCompilerCalls as rustc_driver::Callbacks>::after_analysis
  15: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  16: rustc_span::with_source_map
  17: rustc_interface::interface::create_compiler_and_run
  18: scoped_tls::ScopedKey<T>::set
*/
