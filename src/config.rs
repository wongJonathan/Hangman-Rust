use std::fmt;


pub struct Config{
    pub filename: Option<String>,
}

impl Config {
    pub fn new(raw_args: &[String]) -> Result<Config, &'static str> {
        if raw_args.len() > 2 {
            return Err("Too many arguments. Only 1 optional argument is used.");
        }

        if raw_args.len() == 2 {
            return Ok(Config{ filename: Some(raw_args[1].clone()) });
        }

        Ok(Config{ filename: None })

    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
        .field("filename", &self.filename)
        .finish()
    }
}