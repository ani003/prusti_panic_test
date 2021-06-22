// thread 'rustc' panicked at 'no entry found for key'
// prusti-viper/src/encoder/procedure_encoder.rs:564:13

fn foo(p: Option<i32>) {
    loop {      // Requires both loops
        loop {
            match p {
                None => break,  //Requires this break
                Some(_) => ()
            }
        }
    }
}
 
fn main() {}