use conf;
use request;

pub fn search(api_token: &String, login: &String, options: &conf::SearchOption, curr_depth: u8) -> Option<Box<Vec<String>>> {
    if options.max_depth() == curr_depth {
        return Some(Box::new(Vec::new()));
    }

    let mut res = Vec::new();

    let api_str = "https://api.github.com/users/".to_string();
    let starred_str = &format!("{}{}{}?access_token={}", api_str, login, "/starred".to_string(), api_token);
    let following_str = &format!("{}{}{}?access_token={}", api_str, login, "/following".to_string(), api_token);

    if options.min_depth() >= curr_depth {
        let star = request::get_json(starred_str);
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

    let foll = request::get_json(following_str);
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