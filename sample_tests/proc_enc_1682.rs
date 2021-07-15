fn foo(mut buf: &[u8]){
    while true {
        buf = &buf[1..];
    }
}

fn main() {}