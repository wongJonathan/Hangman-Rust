use std::error::Error;
use std::process;
use std::io;

use crate::config::Config;
use crate::file_reader::{read_file, create_word_list};
use crate::game_logic::Game;


pub fn start(args: &Vec<String>) -> Result<(), Box<dyn Error>>  {
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Trouble with arugments: {}", err);
        println!("Usage: ./hangman [filename]");
        println!("\tfilename: Filename for a csv with words to use for Hangman.");
        process::exit(1);
    });

    let possible_words: Vec<String> = match config.filename {
        Some(filename) => create_word_list(read_file(&filename).unwrap()),
        None => vec![String::from("test")],
    };

    run(&possible_words);

    Ok(())
}

fn run(possible_words: &Vec<String>) {
    let mut games: Vec<Game> = vec!();
    let mut quit = false;

    while !quit {
        print!("\x1B[2J");
        println!("Welcome to a new game of hangman!");
        
        let mut game = Game::create_game(possible_words).unwrap();
        game.display();

        while !game.check_win() {
            let mut guess = String::new();
            
            println!("Guess a letter, or type 'quit' to leave':");
            
            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");
    
            match &guess.to_lowercase().trim()[..] {
                "quit" => {
                    quit = true;
                    break;
                },
                other => if other.len() != 1 {
                    println!("\nInvalid guess at character.\n");
                } else {
                    game.guess(&guess.chars().next().unwrap());
                }
            }
            print!("\x1B[2J");

            game.display();
        }
    
        if game.win {
            println!("Congratulations, you win!");
        } else {
            println!("Sorry you lost :(");
            println!("The word was {}.", game.word);
        }

        games.push(game);

        loop {
            let mut answer = String::new();

            println!("Play another? yes/no");
            
            io::stdin().read_line(&mut answer)
                .expect("Failed to read line");

            match &answer.to_lowercase().trim()[..] {
                "yes" => break,
                "no" => {
                    quit = true;
                    break;
                },
                _ => println!("Please type 'yes' or 'no'."),
            }
        }
    }

    display_stats(&games);
}

fn display_stats(games: &Vec<Game>) {
    let wins = games.iter().fold(
        0, 
        |acc, game| if game.win { acc + 1 } else { acc }
    );
    let guesses = games.iter().fold(0, |acc, game| game.guess_count + acc);
    let wrong_guesses = games.iter().fold(0, |acc, game| {
        println!("{}", game.wrong_count);
        game.wrong_count + acc
    });
    println!("Games played {}", games.len());
    println!("Win Percentage: {:.2}%", (wins as f32)/(games.len() as f32) * 100.00);
    println!("Total guesses: {}", guesses);
    println!("Correct percentage: {:.2}%", (1.00 - (wrong_guesses as f32)/(guesses as f32)) * 100.00);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_creates_game () {

    }

    #[test]
    fn handles_correct_input () {

    }
}