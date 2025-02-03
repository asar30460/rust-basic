use rand::Rng;
use std::io;

fn main() {
    let mut option = String::new();

    println!("Enter option: ");
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..101);

    match option.trim() {
        "1" => println!("Run first function: {}", secret_number),
        _ => println!("Undefined"),
    }
}
