// thread 'rustc' panicked at 'not implemented: elem = Index(_8)'
// analysis/src/abstract_domains/place_utils.rs:117:13

trait Foo: Iterator + Sized {
    fn bar(mut self, buf: &mut [Self::Item]) {
        if let Some(elem) = self.next() {
            buf[0] = elem;
        }
    }
}

fn main() {}
