fn main() {
    let mut v: Vec<i32> = Vec::new();
    let _ = (0..1).map(|_| {
        v = Vec::new();
    });
}

/*
thread 'rustc' panicked at 'not implemented: ty=Closure(DefId(0:4 ~ place_utils_81[5c15]::main::{closure#0}), [i16, extern "rust-call" fn((i32,)), (&mut std::vec::Vec<i32>,)])', analysis/src/abstract_domains/place_utils.rs:81:17
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: analysis::abstract_domains::place_utils::expand_one_level
   3: analysis::abstract_domains::place_utils::expand
   4: analysis::abstract_domains::definitely_initialized::DefinitelyInitializedState::set_place_uninitialised
   5: <analysis::abstract_domains::definitely_initialized::DefinitelyInitializedState as analysis::abstract_state::AbstractState>::apply_terminator_effect
   6: analysis::analyzer::Analyzer::run_fwd_analysis
   7: prusti_interface::environment::mir_analyses::initialization::compute_definitely_initialized
   8: prusti_viper::encoder::initialisation::InitInfo::new
   9: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::new
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
