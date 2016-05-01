use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;

pub struct Config {
    file: String,
    params: HashMap<String, String>
}

impl Config {
    pub fn new(file: String) -> Config {
        Config { file: file, params: HashMap::new() }
    }

    pub fn load(&mut self) -> Result<&Config, String> {
        let f = match fs::File::open(&self.file) {
            Ok(e) => e,
            Err(e) => {
                return Err(format!("Failed to load {}, {}", self.file, e));
            }
        };
        self.params.clear();
        let mut reader = io::BufReader::new(f);
        let mut buffer = String::new();

        loop {
            let result = reader.read_line(&mut buffer);

            if result.is_ok() {
                if result.ok().unwrap() <= 0 {
                    break;
                }
                let mut line = buffer.split("=");

                match (line.next(), line.next()) {
                    (Some(key), Some(value)) => {
                        let key: &str = key; // Needed to explicitly say the type...
                        let value: &str = value;
                        self.params.insert(key.trim().to_string(), value.trim().to_string());
                    },
                    _ => ()
                }
            }

            buffer.clear();
        }

        Ok(self)
    }

    pub fn login(&self) -> String {
        match self.params.get("root-login") {
            Some(e) => e.to_string(),
            None => panic!("No root-login parameter in config file")
        }
    }

    pub fn token(&self) -> String {
        match self.params.get("token") {
            Some(e) => e.to_string(),
            None => panic!("No token parameter in config file")
        }
    }

    pub fn min_depth(&self) -> u8 {
        match self.params.get("depth-min") {
            Some(e) => match e.to_string().parse::<u8>() {
                Ok(v) => v,
                Err(_) => 0
            },
            None => 0
        }
    }

    pub fn max_depth(&self) -> u8 {
        match self.params.get("depth-max") {
            Some(e) => match e.to_string().parse::<u8>() {
                Ok(v) => v,
                Err(_) => 7
            },
            None => 7
        }
    }

    pub fn min_star(&self) -> u32 {
        match self.params.get("star-min") {
            Some(e) => match e.to_string().parse::<u32>() {
                Ok(v) => v,
                Err(_) => 0
            },
            None => 0
        }
    }

    pub fn max_star(&self) -> Option<u32> {
        match self.params.get("star-max") {
            Some(e) => match e.to_string().parse::<u32>() {
                Ok(v) => Some(v),
                Err(_) => None
            },
            None => None
        }
    }

    pub fn languages(&self) -> Option<Vec<String>> {
        match self.params.get("languages") {
            Some(languages) => {
                let mut lang: Vec<String> = Vec::new();
                let line: Vec<&str> = languages.split(",").collect();
                for l in line {
                    lang.push(l.trim().to_string().to_uppercase())
                }
                if lang.len() > 0 {
                    Some(lang)
                } else {
                    None
                }
            },
            None => None
        }
    }
}