extern crate hyper;
extern crate url;
extern crate serde_json;
extern crate rand;

use url::Url;
use hyper::client::Request;
use hyper::method::Method;
use std::io;
use std::fs;
use std::vec::Vec;
use hyper::header;
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

fn get_json(url: &String) -> serde_json::Value {
    let url = match Url::parse(url.as_str()) {
        Ok(url) => url,
        Err(err) => panic!("Failed to parse URL: {}", err)
    };

    let mut req = match Request::new(Method::Get, url) {
        Ok(req) => req,
        Err(err) => panic!("Failed to create Request: {}", err)
    };

    let user_agent = "User-Agent: lpelleau@insa-rennes.fr";
    req.headers_mut().set(header::ContentLength(0u64));
    req.headers_mut().set(header::UserAgent(user_agent.to_string()));

    let req_started = match req.start() {
        Ok(req) => req,
        Err(err) => panic!("Failed to start Request: {}", err)
    };

    let mut resp = match req_started.send() {
        Ok(resp) => resp,
        Err(err) => panic!("Failed to send Request: {}", err)
    };

    let mut resp_body = String::new();
    match resp.read_to_string(&mut resp_body) {
        Ok(_) => (),
        Err(err) => panic!("Failed to parse body to String: {}", err)
    };

    serde_json::from_str(resp_body.as_str())
        .unwrap_or_else(|e| panic!("Failed when getting JSON: {:?} (code: {})", e, resp.status))
}

//fn search<'a>(login: &String, depth: &i8) -> Option<Box<&'a mut Vec<String>>> {
fn search(api_token: &String, login: &String, depth: &i8) -> Option<Box<Vec<String>>> {
    let mut res = Vec::new();

    let api_str = "https://api.github.com/users/".to_string();
    let starred_str = &format!("{}{}{}?access_token={}", api_str, login, "/starred".to_string(), api_token);
    let following_str = &format!("{}{}{}?access_token={}", api_str, login, "/following".to_string(), api_token);

    let star = get_json(starred_str);
    let starred_vec = match star.as_array() {
        Some(x) => x,
        None => panic!("Failure on JSON parsing")
    };

    for repository in starred_vec {
        let starred = repository.as_object()
            .and_then(|object| object.get("html_url"))
            .and_then(|value| value.as_string())
            .unwrap_or_else(|| panic!("Failed to get starred"));
        res.push(starred.to_string());
    }

    if *depth == 1 {
        return Some(Box::new(res));
    }

    let foll = get_json(following_str);
    let following_vec = match foll.as_array() {
        Some(x) => x,
        None => panic!("Failure on JSON parsing")
    };

    for user in following_vec {
        let login = user.as_object()
            .and_then(|object| object.get("login"))
            .and_then(|value| value.as_string())
            .unwrap_or_else(|| panic!("Failed to get following"));

        let sub_res = search(&api_token, &login.to_string(), &(depth - 1));

        if let Some(x) = sub_res {
            unsafe {
                res.append(&mut (*Box::into_raw(x)));
            }
        }
    }

    Some(Box::new(res))
}

fn main() {
    let config = read_config(&"config.ini".to_string());

    if let Some(token) = config.get("token") {
        if let Some(login) = config.get("root-login") {
            if let Some(all_repo) = search(token, login, &3) {
                let selected = rand::random::<usize>() % all_repo.len();

                if let Some(repo) = all_repo.get(selected) {
                    println!("{}", repo);
                }
            }
        }
    }
}
