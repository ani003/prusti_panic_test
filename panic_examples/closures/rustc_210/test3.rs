// thread 'rustc' panicked at 'DefId::expect_local: `DefId(2:2462 ~ core[ec89]::clone::Clone::clone)` isn't local'
// /rustc/ed597e7e19d0fe716d9f81b1e840a5abbfd7c28d/compiler/rustc_span/src/def_id.rs:210:43

fn foo(r: Vec<usize>) {
    let _ = r.iter().map(Clone::clone);
}

fn main() {}
