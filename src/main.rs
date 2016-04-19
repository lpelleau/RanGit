extern crate hyper;
extern crate url;

use std::io;
use std::fs;
use std::vec::Vec;
use std::io::prelude::*;
use std::collections::HashMap;

fn read_config(file: &String) -> Box<HashMap<String, String>> {
    let f = match fs::File::open(file) {
        Ok(e) => e,
        Err(e) => panic!("{}", e)
    };
    let mut reader = io::BufReader::new(f);
    let mut buffer = String::new();

    let mut res = HashMap::new();

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

fn main() {
    let config = read_config(&"config.ini".to_string());
    for (key, value) in config .iter() {
        println!("{}: {}", key, value);
    };
}
