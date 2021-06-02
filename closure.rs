use std::mem;

// A function which takes a closure as an argument and calls it
// <F> denotes that F is a "generic type parameter"
fn apply<F>(f: F) where
F: FnOnce() {
    // try changing to Fn or FnMut

    f();
}

// A function which takes a closure and returns a i32
fn apply_to_3<F>(f: F) -> i32 where
    // the closure takes an i32 and returns an i32
F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    let greeting = "hello";
    // a non-copy type
    // 'to_owned' creates owned data from borrowed
    let mut farewell = "goodbye".to_owned();

    // capture 2 variables: 'greeting' by reference and
    // 'farewell' by value
    let diary = || {
        // 'greeting' is by reference: requires 'Fn'
        println!("I said {}.", greeting);

        // mutation forces 'farewell' to be captured by
        // mutable reference
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzz");

        mem::drop(farewell);
    };

    apply(diary);

    // double satisfies 'apply_to_3's trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

