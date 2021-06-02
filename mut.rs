#[derive(Clone, Copy, Debug)]
enum InvStatus {
    Available,
    Unavailable,
    Archive
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Book {
    // &'static str is a reference to a string allocated in read-only memory
    author: &'static str,
    title: &'static str,
    year: u32,
    inventory: InvStatus
}

// this function takes a reference to a book
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// this function takes a reference to a mutable book and changes year
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

impl Book {
    pub fn new(author: &'static str, title: &'static str, year: u32) -> Book {
        Book {
            author,
            title,
            year,
            inventory: InvStatus::Available
        }
    }

    pub fn retire(&mut self) {
        self.inventory = InvStatus::Archive;
    }

    pub fn checkout(&mut self) {
        self.inventory = InvStatus::Unavailable
    }

    pub fn status(&self) {
        match self.inventory {
            InvStatus::Available => println!("This book is available!"),
            InvStatus::Unavailable => println!("This book is unavailable :("),
            _ => println!("This book has been archived and is no longer in circulation.")
        }
    }
}

fn main() {
    // create an immutable Book named 'ibook'
    let ibook = Book {
        author: "Franz Kafka",
        title: "The Trial",
        year: 1930,
        inventory: InvStatus::Available
    };
    
    let mut book2 = Book::new("John Steinbeck", "The Grapes of Wrath", 1932);
    println!("Book 2: {:?}", book2);

    book2.checkout();
    println!("Book 2: {:?}", book2);
    book2.status();

    // create a mutable copy of 'ibook' and call it 'mbook'
    let mut mbook = ibook;

    // immutably borrow an immutable object
    borrow_book(&ibook);

    // immutably borrow a mutable object
    borrow_book(&mbook);

    // borrow a mutable object as mutable
    new_edition(&mut mbook);

    book2.retire();
    book2.status();

    // cannot borrow immutable object as mutable
    //new_edition(&mut ibook);
}

