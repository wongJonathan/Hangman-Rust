use std::fs;
use std::io;


pub fn read_file(filename: &String) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}


pub fn create_word_list(file: String) -> Vec<String> {
    file.split(",").map(|s| String::from(s.trim())).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_words_correctly () {
        let mock_file = String::from("1,2,3");
        let expected: Vec<String> = vec![String::from("1"), String::from("2"), String::from("3")];

        assert_eq!(create_word_list(mock_file), expected);
    }
}