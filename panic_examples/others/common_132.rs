enum Path<'a> {
    Map { parent: &'a Path<'a> },
    Unknown { parent: &'a Path<'a> },
}

struct Deserializer<'a> {
    path: Path<'a>,
}

impl<'a> Deserializer<'a> {
    fn next_value_seed(&self)
    {
        let mut value_de = Deserializer {
            path: if true {
                Path::Map {
                    parent: &self.path,
                }
            } else {
                Path::Unknown {
                    parent: &self.path,
                }
            }
        };
        let _ = foo(&mut value_de);
    }
}

fn foo<'a>(_: &'a mut Deserializer<'a>){}

fn main() {}

/*
thread 'rustc' panicked at 'Undefined comparison between Remaining and Write', vir/src/legacy/ast/common.rs:132:33
stack backtrace:
   0: rust_begin_unwind
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/std/src/panicking.rs:517:5
   1: core::panicking::panic_fmt
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/panicking.rs:93:14
   2: core::option::expect_failed
             at /rustc/8007b506ac5da629f223b755f5a5391edd5f6d01/library/core/src/option.rs:1618:5
   3: <vir::legacy::ast::common::PermAmount as core::cmp::Ord>::cmp
   4: <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::fold
   5: prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt::obtain
   6: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold
   7: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
   8: prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt::obtain_permissions
   9: <prusti_viper::encoder::foldunfold::FoldUnfold as vir::legacy::cfg::visitor::CfgReplacer<prusti_viper::encoder::foldunfold::path_ctxt::PathCtxt,prusti_viper::encoder::foldunfold::ActionVec>>::replace_stmt
  10: prusti_viper::encoder::foldunfold::add_fold_unfold
  11: prusti_viper::encoder::procedure_encoder::ProcedureEncoder::encode
  12: prusti_viper::encoder::encoder::Encoder::encode_procedure
  13: prusti_viper::encoder::encoder::Encoder::process_encoding_queue
  14: prusti_viper::verifier::Verifier::verify
  15: prusti_driver::verifier::verify
  16: <prusti_driver::callbacks::PrustiCompilerCalls as rustc_driver::Callbacks>::after_analysis
  17: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  18: rustc_span::with_source_map
  19: rustc_interface::interface::create_compiler_and_run
  20: scoped_tls::ScopedKey<T>::set
*/

