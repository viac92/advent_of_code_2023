use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let file_path = "src/puzzle_input.txt";
    let file = File::open(file_path)?;  
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let mut number_string = String::new();
        for character in line.chars() {
            if character.is_numeric() {
                number_string.push(character);
                break;
            }
        }
        for character in line.chars().rev() {
            if character.is_numeric() {
                number_string.push(character);
                break;
            }
        }           
        println!("{}", number_string); 

        match number_string.parse::<i32>() {
            Ok(number) => sum += number,
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}
