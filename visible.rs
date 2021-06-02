mod my {
    // public struct with public field type 'T'
    pub struct OpenBox<T> {
        pub contents: T
    }

    // public struct with private field of type 'T'
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T
    }

    impl<T> ClosedBox<T> {
        // constructor
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents
            }
        }
    }
}

fn main() {
    let open_box = my::OpenBox { contents: "public information" };

    println!("The open box contains: {}", open_box.contents);

    //let closed_box = my::ClosedBox { contents: "classified information" };

    // structs with private fields can be created with
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    //println!("The closed box contains: {}", _closed_box.contents);

}
