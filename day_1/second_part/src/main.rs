use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let file_path = "src/puzzle_input.txt";
    let file = File::open(file_path)?;  
    let reader = BufReader::new(file);

    let mut sum = 0;
    
    let mut string_start = String::new();
    let mut string_end = String::new();

    for line in reader.lines() {
        let line = line?;



        let mut number_string = String::new();
        for character in line.chars() {

            let (number, is_number) = check_word_number(string_start.clone());
            if is_number {
                string_start.clear();
                number_string.push(number);
                break;
            }
            if character.is_numeric() {
                number_string.push(character);
                break;
            } else {
                string_start.push(character);
            } 

        }

        for character in line.chars().rev() {
            let (number, is_number) = check_word_number(string_end.clone());
            if is_number {
                string_end.clear();
                number_string.push(number);
                break;
            }
            if character.is_numeric() {
                number_string.push(character);
                break;
            } else {
                let mut new_s = String::new();
                new_s.push(character);
                new_s.push_str(&string_end);
                string_end = new_s;
                println!("{}", string_end)
            }

        }           
        println!("{}", number_string); 

        match number_string.parse::<i32>() {
            Ok(number) => sum += number,
            Err(e) => println!("Error: {}", e),
        }
    }

    fn check_word_number(line: String) -> (char, bool) {
       let mut c = '0';
       let mut is_number = true;
       match line {
            line if line.contains("one") => c = '1',
            line if line.contains("two") => c = '2',
            line if line.contains("three") => c = '3',
            line if line.contains("four") => c = '4',
            line if line.contains("five") => c = '5',
            line if line.contains("six") => c = '6',
            line if line.contains("seven") => c = '7',
            line if line.contains("eight") => c = '8',
            line if line.contains("nine") => c = '9',
            _ => is_number = false,
        }
        (c, is_number)
    }

    println!("Sum: {}", sum);

    Ok(())
}

