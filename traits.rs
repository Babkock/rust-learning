struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Static method signature; 'self' refers to the implementor type
    fn new(name: &'static str) -> Self;

    // Instance method signatures. These return strings
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        }
        else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// implement the Animal trait for Sheep
impl Animal for Sheep {
    // 'Self' is the implementor type: Sheep
    fn new(name: &'static str) -> Sheep {
        Sheep { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "Baaa"
        }
        else {
            "Baa Baaaa"
        }
    }

    // default trait methods can be overriden
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

