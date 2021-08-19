#[derive(PartialEq)]
struct Foo {
    a: [[[u32; 4]; 8]; 32],
}

fn main() {}

/*
[2021-08-12T07:59:06Z ERROR viper::verifier] The verification aborted due to the following exception: java.lang.RuntimeException: Domain name Snap$Array$4$u32 not found in program.
        at scala.sys.package$.error(package.scala:27)
        at viper.silver.ast.Program.findDomain(Program.scala:277)
        at viper.silver.ast.utility.DomainInstances$DomainInstanceTypeCoordinate.collectTypes(DomainInstances.scala:474)
        at viper.silver.ast.utility.DomainInstances$.findNecessaryTypeInstances(DomainInstances.scala:242)
        at viper.silver.ast.Program.groundTypeInstances$lzycompute(Program.scala:240)
        at viper.silver.ast.Program.groundTypeInstances(Program.scala:240)
        at viper.silicon.supporters.BuiltinDomainsContributor.computeGroundTypeInstances(BuiltinDomainsContributor.scala:95)
        at viper.silicon.supporters.BuiltinDomainsContributor.analyze(BuiltinDomainsContributor.scala:58)
        at viper.silicon.verifier.DefaultMasterVerifier.$anonfun$analyzeProgramAndEmitPreambleContributions$1(DefaultMasterVerifier.scala:397)
        at viper.silicon.verifier.DefaultMasterVerifier.$anonfun$analyzeProgramAndEmitPreambleContributions$1$adapted(DefaultMasterVerifier.scala:396)
        at scala.collection.immutable.List.foreach(List.scala:333)
        at viper.silicon.verifier.DefaultMasterVerifier.analyzeProgramAndEmitPreambleContributions(DefaultMasterVerifier.scala:396)
        at viper.silicon.verifier.DefaultMasterVerifier.verify(DefaultMasterVerifier.scala:177)
        at viper.silicon.Silicon.viper$silicon$Silicon$$runVerifier(Silicon.scala:245)
        at viper.silicon.Silicon$$anon$1.call(Silicon.scala:198)
        at viper.silicon.Silicon$$anon$1.call(Silicon.scala:197)
        at java.base/java.util.concurrent.FutureTask.run(FutureTask.java:264)
        at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(ThreadPoolExecutor.java:1128)
        at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(ThreadPoolExecutor.java:628)
        at java.base/java.lang.Thread.run(Thread.java:829)
    
thread 'rustc' panicked at 'internal error: entered unreachable code: The verifier returned an unknown error of type viper.silver.verifier.AbortedExceptionally: Verification aborted exceptionally (<no position>)', viper/src/verifier.rs:205:21
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4e282795d7d1d28a4c6c1c6521045ae2b59f3519/library/std/src/panicking.rs:516:5
   1: core::panicking::panic_fmt
             at /rustc/4e282795d7d1d28a4c6c1c6521045ae2b59f3519/library/core/src/panicking.rs:93:14
   2: viper::verifier::Verifier<viper::verifier::state::Started>::verify
   3: prusti_server::verifier_runner::VerifierRunner::verify
   4: prusti_server::verifier_runner::VerifierRunner::with_default_configured_runner
   5: prusti_viper::verifier::Verifier::verify
   6: prusti_driver::verifier::verify
   7: <prusti_driver::callbacks::PrustiCompilerCalls as rustc_driver::Callbacks>::after_analysis
   8: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
   9: rustc_span::with_source_map
  10: rustc_interface::interface::create_compiler_and_run
  11: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
*/
