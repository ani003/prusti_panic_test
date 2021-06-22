// thread 'rustc' panicked at 'DefId::expect_local: `DefId(2:39957 ~ core[ec89]::option::Option::Some::{constructor#0})` isn't local'
// /rustc/ed597e7e19d0fe716d9f81b1e840a5abbfd7c28d/compiler/rustc_span/src/def_id.rs:210:43

fn main() {
    let _ = (0..1).map(Some);
}
 