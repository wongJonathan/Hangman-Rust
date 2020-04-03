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
                    println!("\nInvalid guess at character. Please enter one letter or 'quit' to leave.\n");
                } else {
                    game.guess(&guess.chars().next().unwrap());
                }
            }

            game.display();
        }

        if !quit {
            quit = end_game(&game);
        }

        games.push(game);
    }

    display_stats(&games);
}

#[derive(Debug)]
struct EndGameStats {
    wins: i32,
    total_guesses: i32,
    total_wrong: i32,
}

fn display_stats(games: &Vec<Game>) {
    let game_stats: EndGameStats = games.iter().fold(
        EndGameStats {
            wins: 0, 
            total_guesses: 0, 
            total_wrong: 0
        }, 
        |acc, game| {
            EndGameStats{
                wins: if game.win { acc.wins + 1 } else { acc.wins }, 
                total_guesses: game.guess_count + acc.total_guesses, 
                total_wrong: game.wrong_count + acc.total_wrong, 
            }
        }
    );
    println!("Games played {}", games.len());
    println!("Win Percentage: {:.2}%", (game_stats.wins as f32)/(games.len() as f32) * 100.00);
    println!("Total guesses: {}", game_stats.total_guesses);
    println!("Correct percentage: {:.2}%", 
        (game_stats.total_guesses - game_stats.total_wrong) as f32/(game_stats.total_guesses as f32) * 100.00
    );
}

fn end_game(game: &Game) -> bool {
    if game.win {
        println!("Congratulations, you win!");
    } else {
        println!("Sorry you lost :(");
        println!("The word was {}.", game.word);
    }


    loop {
        let mut answer = String::new();

        println!("Play another? yes/no");
        
        io::stdin().read_line(&mut answer)
            .expect("Failed to read line");

        match &answer.to_lowercase().trim()[..] {
            "yes" => return false,
            "no" => return true,
            _ => println!("Please type 'yes' or 'no'."),
        }
    }
}
