// thread 'rustc' panicked at 'DefId::expect_local: `DefId(2:25187 ~ core[ec89]::num::{impl#11}::checked_next_power_of_two)` isn't local'
// /rustc/ed597e7e19d0fe716d9f81b1e840a5abbfd7c28d/compiler/rustc_span/src/def_id.rs:210:43

fn foo(x: &Option<usize>) {
    let _ = x.and_then(usize::checked_next_power_of_two);
}

fn main() {}
