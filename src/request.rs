extern crate hyper;
extern crate url;
extern crate serde_json;

use url::Url;
use hyper::client::Request;
use hyper::method::Method;
use hyper::header;
use std::io::prelude::*;

pub struct APIRest {
    url: String,
    user_agent: String
}

impl APIRest {
    pub fn new(url: String) -> APIRest {
        APIRest { url: url, user_agent: "".to_string() }
    }

    pub fn set_user_agent(&mut self, user_agent: String) -> &APIRest {
        self.user_agent = user_agent;
        self
    }

    pub fn get(&self, uri: &String) -> Result<serde_json::Value, String> {
        let url = match Url::parse(format!("{}{}", self.url, uri).as_str()) {
            Ok(url) => url,
            Err(err) => return Err(format!("Failed to parse URL: {}", err))
        };

        let mut req = match Request::new(Method::Get, url) {
            Ok(req) => req,
            Err(err) => return Err(format!("Failed to create Request: {}", err))
        };

        req.headers_mut().set(header::ContentLength(0u64));
        req.headers_mut().set(header::UserAgent(self.user_agent.clone()));

        let req_started = match req.start() {
            Ok(req) => req,
            Err(err) => return Err(format!("Failed to start Request: {}", err))
        };

        let mut resp = match req_started.send() {
            Ok(resp) => resp,
            Err(err) => return Err(format!("Failed to send Request: {}", err))
        };

        let mut resp_body = String::new();
        match resp.read_to_string(&mut resp_body) {
            Ok(_) => (),
            Err(err) => return Err(format!("Failed to parse body to String: {}", err))
        };

        match serde_json::from_str(resp_body.as_str()) {
            Ok(json) => Ok(json),
            Err(err) => Err(format!("Failed when getting JSON: {:?} (code: {})", err, resp.status))
        }
    }
}