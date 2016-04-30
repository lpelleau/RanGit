use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;

pub struct Config<'a> {
    file: &'a str,
    params: HashMap<String, String>
}

impl<'a> Config<'a> {
    pub fn new(file: &str) -> Config {
        Config { file: file, params: HashMap::new() }
    }

    pub fn load(&mut self) -> () {
        let f = match fs::File::open(self.file) {
            Ok(e) => e,
            Err(e) => {
                println!("Failed to load {}, {}", self.file, e);
                return;
            }
        };
        let mut reader = io::BufReader::new(f);
        let mut buffer = String::new();

        loop {
            let result = reader.read_line(&mut buffer);

            if result.is_ok() {
                if result.ok().unwrap() <= 0 {
                    break;
                }
                let mut line = buffer.split("=");

                let key = line.next().unwrap().trim().to_string();
                let value = line.next().unwrap().trim().to_string();

                self.params.insert(key, value);
            }

            buffer.clear();
        }
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
            Some(e) => e.to_string().parse::<u8>().unwrap(),
            None => 0
        }
    }

    pub fn max_depth(&self) -> u8 {
        match self.params.get("depth-max") {
            Some(e) => e.to_string().parse::<u8>().unwrap(),
            None => 7
        }
    }

    pub fn min_star(&self) -> u32 {
        match self.params.get("star-min") {
            Some(e) => e.to_string().parse::<u32>().unwrap(),
            None => 0
        }
    }

    pub fn max_star(&self) -> Option<u32> {
        match self.params.get("star-max") {
            Some(e) => Some(e.to_string().parse::<u32>().unwrap()),
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
                Some(lang)
            },
            None => None
        }
    }
}