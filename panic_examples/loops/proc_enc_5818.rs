// thread 'rustc' panicked at 'no entry found for key'
// prusti-viper/src/encoder/procedure_encoder.rs:5818:10

enum Foo {
    Bar,
}

fn abc(x: &Foo) {
    let mut p = x;
    while let Foo::Bar = *p {
        p = x;
    }
}

fn main() {}

// Linked list
/*
enum LL<'a> {
    Null,
    Node(i32, &'a LL<'a>)
}

fn traverse(head: &LL<'_>) {
    let mut nd = head;
    while let LL::Node(_, succ) = *nd {
        nd = succ;
    }
}

fn main() {}
*/
