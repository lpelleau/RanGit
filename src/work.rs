use conf;
use request;

pub fn search(options: &conf::Config, api: &request::APIRest, login: &String, curr_depth: u8) -> Option<Box<Vec<String>>> {
    if options.max_depth() == curr_depth {
        return Some(Box::new(Vec::new()));
    }

    let mut res = Vec::new();

    let api_starred = &format!("{}{}?access_token={}", login, "/starred".to_string(), options.token());
    let api_following = &format!("{}{}?access_token={}", login, "/following".to_string(), options.token());

    if options.min_depth() >= curr_depth {
        if let Ok(star) = api.get(api_starred) {
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
    }

    if options.max_depth() == curr_depth + 1 {
        return Some(Box::new(res));
    }

    if let Ok(foll) = api.get(api_following) {
        let following_vec = match foll.as_array() {
            Some(x) => x,
            None => panic!("Failure on JSON parsing")
        };

        for user in following_vec {
            let login = user.as_object()
                .and_then(|object| object.get("login"))
                .and_then(|value| value.as_string())
                .unwrap_or_else(|| panic!("Failed to get following"));

            let sub_res = search(options, api, &login.to_string(), curr_depth + 1);

            if let Some(x) = sub_res {
                unsafe {
                    res.append(&mut (*Box::into_raw(x)));
                }
            }
        }
    }

    Some(Box::new(res))
}