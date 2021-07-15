// thread 'rustc' panicked at 'not implemented: NonMutatingUse(AddressOf)'
// prusti-interface/src/environment/loops.rs:117:22

use std::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};
static HASHTABLE: AtomicPtr<i32> = AtomicPtr::new(ptr::null_mut());

fn get_hashtable() -> &'static i32 {
    unsafe { &*HASHTABLE.load(Ordering::Acquire) }
}

fn grow_hashtable() {   // Must be as it is. Cannot simplify
    loop {
        let table = get_hashtable();
        
        if true {
            break;
        }

        let _ = table as *const _ as *mut AtomicPtr<i32>;
    }
}

fn main() {}
