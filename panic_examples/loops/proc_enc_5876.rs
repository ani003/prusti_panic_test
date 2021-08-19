enum Foo {
    Bar,
}

fn abc(x: &Foo) {
    let mut p = x;
    while let Foo::Bar = *p {
        p = x;
    }
}

fn main() {}

/*
thread 'rustc' panicked at 'no entry found for key', prusti-viper/src/encoder/procedure_encoder.rs:5876:10
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: core::option::expect_failed
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/option.rs:1618:5
   3: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::construct_vir_reborrowing_dag
   4: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_expiration_of_loans
   5: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_blocks_group
   6: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_loop
   7: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_blocks_group
   8: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode
   9: prusti_viper::encoder::encoder::Encoder::encode_procedure
  10: prusti_viper::encoder::encoder::Encoder::process_encoding_queue
  11: prusti_viper::verifier::Verifier::verify
  12: prusti_driver::verifier::verify
  13: <prusti_driver::callbacks::PrustiCompilerCalls as rustc_driver::Callbacks>::after_analysis
  14: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  15: rustc_span::with_source_map
  16: rustc_interface::interface::create_compiler_and_run
  17: scoped_tls::ScopedKey<T>::set
*/
