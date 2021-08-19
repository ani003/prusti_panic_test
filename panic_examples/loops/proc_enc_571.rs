fn foo(p: Option<i32>) {
    loop {      // Requires both loops
        loop {
            match p {
                None => break,  //Requires this break
                Some(_) => ()
            }
        }
    }
}
 
fn main() {}

/*
thread 'rustc' panicked at 'no entry found for key', prusti-viper/src/encoder/procedure_encoder.rs:571:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: core::option::expect_failed
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/option.rs:1618:5
   3: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_blocks_group
   4: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_loop
   5: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_blocks_group
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
