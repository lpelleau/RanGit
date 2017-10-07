use serde_json;
use url::Url;
use hyper::client::Request;
use hyper::method::Method;
use hyper::header;
use std::io::prelude::*;
use cache::Cache;

pub struct APIRest {
    url: String,
    user_agent: String,
    cache: Cache<String, Result<serde_json::Value, String>>,
}

impl APIRest {
    pub fn new(url: String) -> APIRest {
        APIRest { url: url, user_agent: "".to_string(), cache: Cache::load() }
    }

    pub fn set_user_agent(&mut self, user_agent: String) -> &APIRest {
        self.user_agent = user_agent;
        self
    }

    pub fn get(&mut self, uri: &String) -> Result<serde_json::Value, String> {
        let url = format!("{}{}", self.url, uri);
        let user_agent = self.user_agent.clone();

        let result = self.cache.get_or_compute(url.clone(), || {
            let url = match Url::parse(url.as_str()) {
                Ok(url) => url,
                Err(err) => return Err(format!("Failed to parse URL: {}", err))
            };

            let mut req = match Request::new(Method::Get, url) {
                Ok(req) => req,
                Err(err) => return Err(format!("Failed to create Request: {}", err))
            };

            req.headers_mut().set(header::ContentLength(0u64));
            req.headers_mut().set(header::UserAgent(user_agent));

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
        });

        result.clone()
    }
}

impl Drop for APIRest {
    fn drop(&mut self) {
        self.cache.save();
    }
}
