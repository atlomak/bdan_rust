use rand::Rng; // Import Rng trait to use random number generation methods.
use std::fs::File; // Import File for file handling.
use std::io::prelude::*; // Import traits for writing to files.

fn main() -> std::io::Result<()> {
    let resources_path = String::from("resources/");
    let mut file = File::create(resources_path + "random_numbers.txt")?; // Create a file to save the random numbers.

    let mut rng = rand::thread_rng(); // Create a random number generator.
    let numbers: Vec<u32> = (0..100).map(|_| rng.gen()).collect(); // Generate 100 random numbers and collect them into a vector.

    for number in numbers {
        writeln!(file, "{}", number)?; // Write each number to the file followed by a newline.
    }

    Ok(())
}
