// thread 'rustc' panicked at 'Adding a write root node to an existing tree.'
// prusti-interface/src/environment/loops_utils.rs:282:13

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