use std::cell::Cell;

type Bar = fn(&dyn FnMut()) -> u32;

fn foo(f: &dyn FnMut()) -> u32 { 1 }

// Panic caused by this block
thread_local! {
    static FOO: Cell<Bar> = Cell::new(foo);
}

fn main() {}

/*
thread 'rustc' panicked at 'internal error: entered unreachable code: &/*tls*/ FOO::__getit::__KEY', prusti-interface/src/environment/polonius_info.rs:613:22
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: prusti_interface::environment::polonius_info::get_borrowed_places
   3: prusti_interface::environment::polonius_info::PoloniusInfo::new
   4: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode
   5: prusti_viper::encoder::encoder::Encoder::encode_procedure
   6: prusti_viper::encoder::encoder::Encoder::process_encoding_queue
   7: prusti_viper::verifier::Verifier::verify
   8: prusti_driver::verifier::verify
   9: <prusti_driver::callbacks::PrustiCompilerCalls as rustc_driver::Callbacks>::after_analysis
  10: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  11: rustc_span::with_source_map
  12: rustc_interface::interface::create_compiler_and_run
  13: scoped_tls::ScopedKey<T>::set
*/
