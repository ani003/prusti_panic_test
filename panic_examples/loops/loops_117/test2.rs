fn main() {
    loop {
        match Some(1) {
            Some(c) if c <= 1 => (),
            _ => (),
        }
    }
}

/*
thread 'rustc' panicked at 'not implemented: NonMutatingUse(ShallowBorrow)', prusti-interface/src/environment/loops.rs:117:22
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: <prusti_interface::environment::loops::AccessCollector as rustc_middle::mir::visit::Visitor>::visit_place
   3: prusti_interface::environment::loops::ProcedureLoops::compute_read_and_write_leaves
   4: prusti_viper::encoder::loop_encoder::LoopEncoder::compute_loop_invariant
   5: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_loop_invariant_permissions
   6: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_loop_invariant_exhale_stmts
   7: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_loop
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
