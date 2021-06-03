fn repeat(f: impl Fn(usize), n: usize) {
    (0..n).for_each(f);
}

trait Bar {
    fn run(self);
}
 
impl Bar for usize {
    fn run(self) {
        print!("{};", self);
    }
}
 
fn main() {
    repeat(Bar::run, 6);
}