use std::io;

// this function takes ownership of a box and destroys it
fn eat_box(boxed_i32: Box<i32>) {
    println!("Destroying box that contains: {}", boxed_i32);
}

// this function borrows an i32
fn borrow(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() -> io::Result<()> {
    // create a boxed (heap-allocated) i32, and a stack i32
    let ibox = Box::new(5_i32);
    let istack = 6_i32;

    // borrow the contents of the box. Ownership is not taken
    // so the contents can be borrowed again
    borrow(&ibox);
    borrow(&istack);

    {
        // make a reference to data contained inside the box
        let _iref: &i32 = &ibox;

        // error! can't destroy ibox while the inner value is borrowed later in scope
        //eat_box(ibox);

        // attempt to borrow _iref after inner value is destroyed
        borrow(_iref);
        // _iref goes out of scope and is no longer borrowed
    }

    // ibox can now give up ownership to eat_box and be destroyed
    eat_box(ibox);

    Ok(())
}

