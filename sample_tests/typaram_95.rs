// FIXED

fn foo<T: Copy>(v: &[T], idx: &[usize]) {
    let _ = idx.iter().map(|_| v[0]);
}

fn main() {}