const fn foo() {}

fn main() {}

/*
thread 'rustc' panicked at 'attempted to read from stolen value', /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/compiler/rustc_data_structures/src/steal.rs:37:21
stack backtrace:
   0: std::panicking::begin_panic
   1: prusti_interface::environment::Environment::local_mir
   2: prusti_viper::encoder::specs_closures_collector::SpecsClosuresCollector::collect
   3: prusti_viper::encoder::encoder::Encoder::encode_procedure
   4: prusti_viper::encoder::encoder::Encoder::process_encoding_queue
   5: prusti_viper::verifier::Verifier::verify
   6: prusti_driver::verifier::verify
   7: <prusti_driver::callbacks::PrustiCompilerCalls as rustc_driver::Callbacks>::after_analysis
   8: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
   9: rustc_span::with_source_map
  10: rustc_interface::interface::create_compiler_and_run
  11: scoped_tls::ScopedKey<T>::set
*/
