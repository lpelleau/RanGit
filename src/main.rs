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

fn get_json(url: &String) -> serde_json::Value {
    let url = match Url::parse(url.as_str()) {
        Ok(url) => url,
        Err(err) => panic!("Failed to parse URL: {}", err)
    };

    let mut req = match Request::new(Method::Get, url) {
        Ok(req) => req,
        Err(err) => panic!("Failed to create Request: {}", err)
    };

    let user_agent = "User-Agent: lpelleau/Rangit";
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

fn search(api_token: &String, login: &String, options: &SearchOption, curr_depth: u8) -> Option<Box<Vec<String>>> {
    if options.max_depth() == curr_depth {
        return Some(Box::new(Vec::new()));
    }

    let mut res = Vec::new();

    let api_str = "https://api.github.com/users/".to_string();
    let starred_str = &format!("{}{}{}?access_token={}", api_str, login, "/starred".to_string(), api_token);
    let following_str = &format!("{}{}{}?access_token={}", api_str, login, "/following".to_string(), api_token);

    if options.min_depth() >= curr_depth {
        let star = get_json(starred_str);
        let starred_vec = match star.as_array() {
            Some(x) => x,
            None => panic!("Failure on JSON parsing")
        };

        for repository in starred_vec {
            let rep = repository.as_object();

            let stargazers_count = rep.and_then(|object| object.get("stargazers_count"))
                .and_then(|value| value.as_i64())
                .unwrap_or_else(|| panic!("Failed to get repository star's count")) as u32;

            let language = rep.and_then(|object| object.get("language"))
                .and_then(|value| value.as_string())
                .unwrap_or_else(|| "No language (in API)");

            if let Some(languages) = options.languages() {
                if !languages.contains(&language.to_string().to_uppercase()) {
                    continue;
                }
            }

            let starred = rep.and_then(|object| object.get("html_url"))
                .and_then(|value| value.as_string())
                .unwrap_or_else(|| panic!("Failed to get starred"));

            if options.max_star().is_some() {
                let max = options.max_star().unwrap();

                if stargazers_count > options.min_star() && stargazers_count < max {
                    res.push(starred.to_string());
                }
            } else {
                if stargazers_count > options.min_star() {
                    res.push(starred.to_string());
                }
            }
        }
    }

    if options.max_depth() == curr_depth + 1 {
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

        let sub_res = search(&api_token, &login.to_string(), options, curr_depth + 1);

        if let Some(x) = sub_res {
            unsafe {
                res.append(&mut (*Box::into_raw(x)));
            }
        }
    }

    Some(Box::new(res))
}

struct SearchOption<'a> {
    min_depth: Option<&'a String>,
    max_depth: Option<&'a String>,
    min_star: Option<&'a String>,
    max_star: Option<&'a String>,
    languages: Option<&'a String>
}

impl<'a> SearchOption<'a> {
    fn min_depth(&self) -> u8 {
        match self.min_depth {
            Some(e) => e.parse::<u8>().unwrap(),
            None => 0
        }
    }

    fn max_depth(&self) -> u8 {
        match self.max_depth {
            Some(e) => e.parse::<u8>().unwrap(),
            None => 7
        }
    }

    fn min_star(&self) -> u32 {
        match self.min_star {
            Some(e) => e.parse::<u32>().unwrap(),
            None => 0
        }
    }

    fn max_star(&self) -> Option<u32> {
        match self.max_star {
            Some(e) => Some(e.parse::<u32>().unwrap()),
            None => None
        }
    }

    fn languages(&self) -> Option<Vec<String>> {
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

fn main() {
    let config = read_config(&"config.ini".to_string());

    if let Some(token) = config.get("token") {
        if let Some(login) = config.get("root-login") {
            let options = SearchOption {
                min_depth: config.get("depth-min"),
                max_depth: config.get("depth-max"),
                min_star: config.get("star-min"),
                max_star: config.get("star-max"),
                languages: config.get("languages"),
            };

            if let Some(all_repo) = search(token, login, &options, 0) {
                if all_repo.len() > 0 {
                    let selected = rand::random::<usize>() % all_repo.len();

                    if let Some(repo) = all_repo.get(selected) {
                        println!("{}", repo);
                    }
                } else {
                    println!("No repository found with your criteria.")
                }
            }
        }
    }
}
