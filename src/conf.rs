use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;

pub fn read_config(file: &String) -> Box<HashMap<String, String>> {
    let f = match fs::File::open(file) {
        Ok(e) => e,
        Err(e) => panic!("{}", e)
    };
    let mut reader = io::BufReader::new(f);
    let mut buffer = String::new();

    let mut res: HashMap<String, String> = HashMap::new();

    loop {
        match reader.read_line(&mut buffer) {
            Ok(v) => if v <= 0 { break },
            Err(e) => panic!("{}", e)
        }

        {
            let line: Vec<&str> = buffer.split("=").collect();
            let key = line[0].trim().to_string();
            let value = line[1].trim().to_string();
            res.insert(key, value);
        }
        buffer.clear();
    }
    Box::new(res)
}

pub struct SearchOption<'a> {
    pub min_depth: Option<&'a String>,
    pub max_depth: Option<&'a String>,
    pub min_star: Option<&'a String>,
    pub max_star: Option<&'a String>,
    pub languages: Option<&'a String>
}

impl<'a> SearchOption<'a> {
    pub fn min_depth(&self) -> u8 {
        match self.min_depth {
            Some(e) => e.parse::<u8>().unwrap(),
            None => 0
        }
    }

    pub fn max_depth(&self) -> u8 {
        match self.max_depth {
            Some(e) => e.parse::<u8>().unwrap(),
            None => 7
        }
    }

    pub fn min_star(&self) -> u32 {
        match self.min_star {
            Some(e) => e.parse::<u32>().unwrap(),
            None => 0
        }
    }

    pub fn max_star(&self) -> Option<u32> {
        match self.max_star {
            Some(e) => Some(e.parse::<u32>().unwrap()),
            None => None
        }
    }

    pub fn languages(&self) -> Option<Vec<String>> {
        let mut lang: Vec<String> = Vec::new();
        match self.languages {
            Some(languages) => {
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