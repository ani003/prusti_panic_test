// thread 'rustc' panicked at 'bound variables are not supported at DefId(0:4 ~ type_visitor1[317d]::main::{closure#0})'
// prusti-viper/src/utils/type_visitor.rs:286:19

fn main() {
    let _ = (0..1).filter(|_| true);
}