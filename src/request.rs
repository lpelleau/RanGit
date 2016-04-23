extern crate hyper;
extern crate url;
extern crate serde_json;

use url::Url;
use hyper::client::Request;
use hyper::method::Method;
use hyper::header;
use std::io::prelude::*;

pub fn get_json(url: &String) -> serde_json::Value {
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