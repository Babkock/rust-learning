#![allow(dead_code)]

#[derive(Debug)] pub enum Food { Apple, Carrot, Potato }

#[derive(Debug)] pub struct Peeled(Food);
#[derive(Debug)] pub struct Chopped(Food);
#[derive(Debug)] pub struct Cooked(Food);

// Peeling food. If there isn't any, then return None
// Otherwise, return the peeled food
pub fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None
    }
}

// Chopping food. If there isn't any, then return None
// Otherwise, return the chopped food
pub fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None
    }
}

// Cooking food. Here we showcase "map" instead of "match" for case handling
pub fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// A function to peel, chop, and cook food all in sequence
// We chain multiple uses of 'map'
pub fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// Check whether there's food or not before trying to eat it
pub fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I Love {:?}", food),
        None => println!("Oh no I died")
    }
}

pub fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    // process(x) = cook(chop(peel(x)))
    let cooked_potato = process(potato);
    
    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

