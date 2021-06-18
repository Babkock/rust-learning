use std::io;

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() -> io::Result<()> {
    let nlist = vec![34, 50, 25, 100, 65];

    let mut result = largest(&nlist);
    println!("The largest number of {:?} is {}", nlist, result);

    let nlist = vec![102, 34, 10, 500, 600];

    result = largest(&nlist);
    println!("The largest number of {:?} is {}", nlist, result);
    Ok(())
}

