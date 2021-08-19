struct Foo {
    bar: bool,
}

impl Foo {
    fn abc(&mut self) {}

    fn xyz(mut self) {
        loop {
            self.bar = false;    // Panic trigger requires both statements
            let _ = self.abc();
        }
    }
}

fn main() {}

/*
thread 'rustc' panicked at 'Adding a write root node to an existing tree.', prusti-interface/src/environment/loops_utils.rs:282:13
stack backtrace:
   0: std::panicking::begin_panic
   1: prusti_interface::environment::loops_utils::PermissionTree::add
   2: prusti_interface::environment::loops_utils::PermissionForest::new::add_paths
   3: prusti_interface::environment::loops_utils::PermissionForest::new
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
