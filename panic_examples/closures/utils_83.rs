fn foo(_: u32) {}
 
fn main() {
    let n = 0;
    let _ = (0..1).map(|_| {
        loop {
            foo(n);
        }
    });
}

/*
thread 'rustc' panicked at 'not implemented: ty=Closure(DefId(0:5 ~ utils_83[cdad]::main::{closure#0}), [i16, extern "rust-call" fn((i32,)), (&u32,)])', prusti-interface/src/utils.rs:83:17
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: prusti_interface::utils::expand_struct_place
   3: prusti_interface::utils::expand_one_level
   4: prusti_interface::utils::collapse::recurse
   5: prusti_interface::utils::collapse::recurse
   6: prusti_interface::environment::place_set::PlaceSet::insert
   7: prusti_viper::encoder::loop_encoder::LoopEncoder::compute_loop_invariant
   8: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_loop_invariant_permissions
   9: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_loop_invariant_exhale_stmts
  10: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode_loop
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
