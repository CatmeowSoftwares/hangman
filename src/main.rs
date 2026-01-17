use std::{fs::{self, read_to_string}, io,};
use rand::*;
fn main() {
    let mut words = Vec::<String>::new();
    for line in read_to_string("words.txt").unwrap().lines(){
        words.push(line.to_string());
    }
    let mut used_letters = Vec::<char>::new();
    let mut correct_letters = Vec::<char>::new();
    let word_picked = String::from(&words[rand::random_range(0..words.len())]);
    let mut lost = true;
    let mut current_word = String::new();
    let mut temp_word = Vec::<char>::new();
    let mut tries_left: usize = 7;
    for i in 0..word_picked.len() {
        temp_word.push('_');
    }
    current_word = temp_word.iter().collect();
    loop {
        println!("word: {current_word}");
        println!("Enter a letter:");

        let mut input_letter = String::new();
        io::stdin().read_line(&mut input_letter).expect("cannot");
        let has_letter = match input_letter.trim().parse::<char>() {
            Ok(l) => Some(l),
            _ => None,
        };

        if let Some(letter) = has_letter {
            if used_letters.contains(&letter) { println!("already used"); continue;}
            let char_at: usize;
            let mut wrong = true;
            for i in 0..word_picked.len() {
                if word_picked.chars().nth(i).unwrap() == letter {
                    char_at = i;
                    correct_letters.push(letter);
                    println!("Correct!");
                    current_word.replace_range(char_at..char_at + 1, &letter.to_string());

                    wrong = false;
                    break;
                }
            }
                
                if wrong {
                println!("WRONG!");
                tries_left -= 1;

            }
            used_letters.push(letter);

            
            println!("tries left: {tries_left}");
            match tries_left {
                7 => {
                    println!("");
                    println!("");
                    println!("");
                    println!("");
                    println!("");
                },
                6 => {
                    println!("");
                    println!("");
                    println!("");
                    println!("");
                    println!("____");
                },
                5 => {
                    println!("|");
                    println!("|");
                    println!("|");
                    println!("|");
                    println!("|____");
                },
                4 => {
                    println!("|");
                    println!("|/");
                    println!("|");
                    println!("|");
                    println!("|____");
                },
                3 => {
                    println!("|-----");
                    println!("|/");
                    println!("|");
                    println!("|");
                    println!("|____");
                },
                2 => {
                    println!("|----|-");
                    println!("|/   |");
                    println!("|    O");
                    println!("|");
                    println!("|____");
                },
                1 => {

                },
                0 => {
                    println!("|----|-");
                    println!("|/   O");
                    println!("|   /|\\ ");
                    println!("|   / \\");
                    println!("|___");

                },
                _ => {},
            }

            if correct_letters.len() == word_picked.len() {
                lost = false;
                break;
            }
            if tries_left <= 0 {
                break;
            }
        }
    }
    if lost {
        println!("YOU LOSE");
        println!("correct word was: {}", word_picked);
    } else {
        println!("YOU WIN!!!!!!!!!");
        println!("{} was the correct answer", word_picked);

    }

}