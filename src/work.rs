use conf::*;
use request::*;

pub struct Search {
    options: Config,
    api: APIRest
}

impl Search {
    pub fn new(options: Config, api: APIRest) -> Search {
        Search{ options: options, api: api }
    }

    pub fn compute(&self, login: &String) -> Vec<String> {
        self.comp(login, 0)
    }

    fn comp(&self, login: &String, curr_depth: u8) -> Vec<String> {
        let mut res = Vec::new();

        if self.options.max_depth() == curr_depth {
            return res;
        }

        if self.options.min_depth() >= curr_depth {
            res.append(&mut self.starred(login));
        }

        if self.options.max_depth() == curr_depth + 1 {
            return res;
        }

        res.append(&mut self.following(login, curr_depth));

        res
    }

    fn starred(&self, login: &String) -> Vec<String> {
        let mut res = Vec::new();

        let api_starred = &format!("{}{}?access_token={}", login, "/starred".to_string(), self.options.token());

        if let Ok(star) = self.api.get(api_starred) {
            if let Some(starred_vec) = star.as_array() {
                for repository in starred_vec {
                    let rep = repository.as_object();

                    let stargazers_count = rep.and_then(|object| object.get("stargazers_count"))
                        .and_then(|value| value.as_i64());

                    let language = rep.and_then(|object| object.get("language"))
                        .and_then(|value| value.as_string());

                    let starred = rep.and_then(|object| object.get("html_url"))
                        .and_then(|value| value.as_string());

                    match (stargazers_count, starred) {
                        (Some(count), Some(starred)) => {
                            let lang = language.unwrap_or("").to_string().to_uppercase();
                            if self.options.languages().is_some() && !self.options.languages().unwrap().contains(&lang) {
                                continue;
                            }

                            let count = count as u32;

                            match self.options.max_star() {
                                Some(max_star) => {
                                    if count > self.options.min_star() && count < max_star {
                                        res.push(starred.to_string());
                                    }
                                },
                                _ => {
                                    if count > self.options.min_star() {
                                        res.push(starred.to_string());
                                    }
                                }
                            }
                        },
                        _ => ()
                    }
                }
            }
        }

        res
    }

    fn following(&self, login: &String, curr_depth: u8) -> Vec<String> {
        let mut res = Vec::new();

        let api_following = &format!("{}{}?access_token={}", login, "/following".to_string(), self.options.token());

        if let Ok(foll) = self.api.get(api_following) {
            if let Some(following_vec) = foll.as_array() {
                for user in following_vec {
                    if let Some(login) = user.as_object()
                            .and_then(|object| object.get("login"))
                            .and_then(|value| value.as_string()) {

                        res.append(&mut self.comp(&login.to_string(), curr_depth + 1));
                    }
                }
            }
        }

        res
    }
}