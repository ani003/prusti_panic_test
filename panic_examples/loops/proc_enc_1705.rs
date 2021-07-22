// thread 'rustc' panicked at 'no entry found for key'
// prusti-viper/src/encoder/procedure_encoder.rs:1705:38

fn foo(mut buf: &[u8]){
    while true {    // Must be while
        buf = &buf[1..];
    }
}

fn main() {}