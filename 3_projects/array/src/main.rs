use std::io;

fn main() {
    // Index access
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("{}, {}", first, second);

    let a = [1, 2, 3, 4, 5];

    // Invalid array element access
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
