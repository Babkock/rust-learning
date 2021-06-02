use std::thread;
use std::time::Duration;
use close::generate_workout;
use anyhow::Result;

pub fn main() -> Result<()> {
    let simulated_user_value = 30;
    let simulated_random = 3;

    let x = vec![3, 2, 1];
    println!("The X vector: {:?}", x);

    thread::sleep(Duration::from_secs(3));

    generate_workout(simulated_user_value, simulated_random);
    Ok(())
}
