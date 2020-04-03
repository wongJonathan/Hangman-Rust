use rand::{thread_rng};
use rand::seq::SliceRandom;

const HANGMANPICS: [&'static str; 7] = ["
 +---+
 |   |
     |
     |
     |
     |
=========", "
 +---+
 |   |
 O   |
     |
     |
     |
=========", "
 +---+
 |   |
 O   |
 |   |
     |
     |
=========", "
 +---+
 |   |
 O   |
/|   |
     |
     |
=========", "
 +---+
 |   |
 O   |
/|\\  |
     |
     |
=========", "
 +---+
 |   |
 O   |
/|\\  |
/    |
     |
=========", "
 +---+
 |   |
 O   |
/|\\  |
/ \\  |
     |
========="];


#[derive(Debug)]
pub struct Game<'a> {
    pub word: &'a String,
    pub win: bool,
    pub guesses: Vec<char>,
    pub guess_count: i32,
    pub wrong_count: i32,
    pub correct_guesses: Vec<char>,
}

impl Game<'_> {
    pub fn create_game(possible_words: &Vec<String>) -> Result<Game, &'static str> {
        if possible_words.len() < 1 {
            return Err("Game was not given any possible words to choose from. Check file given.");
        }

        let word = possible_words.choose(&mut thread_rng()).unwrap();
        let correct_guesses: Vec<char> = word.chars().map(|_| {
            '_'
        }).collect();

        Ok(Game{ 
            word,
            win: false,
            guesses: vec!(),
            guess_count: 0,
            wrong_count: 0,
            correct_guesses,
         })
    }

    pub fn guess(&mut self, guess: &char) {
        if self.guesses.contains(guess) {
            println!("You've guessed that letter already");
        } else {
            let mut find = false;

            self.guess_count += 1;
            self.guesses.push(guess.clone());

            for (idx, c) in self.word.chars().enumerate() {
                if c == *guess {
                    self.correct_guesses[idx] = guess.clone();
                    find = true;
                }
            }

            if find {
                println!("Correct");
            } else {
                println!("Wrong");
                self.wrong_count += 1;
            }
        }
        
    }

    pub fn display(&self) {
        println!("{}", HANGMANPICS[self.wrong_count as usize]);
        for c in &self.correct_guesses {
            match c {
                '_' => print!("__ "),
                _ => print!("{} ", c),
            }
        }
        println!("");

        println!("Guesses:");
        for guess in &self.guesses {
            print!("{} ", guess);
        }
        println!("\n");
    }

    pub fn check_win(&mut self) -> bool {
        match &self.correct_guesses.iter().find(|c| **c == '_') {
            Some(_) => self.wrong_count >= (HANGMANPICS.len() - 1) as i32,
            None => {
                self.win = true;
                true
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_guesses () {
        let mut game1 = Game {
            word: &"test".to_string(),
            win: false,
            guesses: vec!(),
            guess_count: 0,
            wrong_count: 0,
            correct_guesses: vec!['_','_','_','_'],
        };

        let good_guess = 't';
        let bad_guess = 'a';

        game1.guess(&good_guess);

        assert_eq!(game1.guesses, vec!['t']);
        assert_eq!(game1.guess_count, 1);
        assert_eq!(game1.wrong_count, 0);
        assert_eq!(game1.correct_guesses, vec!['t', '_', '_', 't']);

        game1.guess(&bad_guess);

        assert_eq!(game1.guesses, vec!['t', 'a']);
        assert_eq!(game1.guess_count, 2);
        assert_eq!(game1.wrong_count, 1);
        assert_eq!(game1.correct_guesses, vec!['t', '_', '_', 't']);
    }
}