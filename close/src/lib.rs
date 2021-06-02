use std::thread;
use std::time::Duration;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub calculation: T,
    pub value: Option<u32>
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // calculation: T is a closure type
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
            	// execute closure
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random: u32) {
    let mut result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!("Today, do {} pushups!", result.value(intensity));
        println!("Next, do {} situps!", result.value(intensity));
    } else {
        if random == 3 {
            println!("Take a break today!");
        }
        else {
            println!(
                "Today, run for {} minutes!",
                result.value(intensity)
            );
        }
    }
}
