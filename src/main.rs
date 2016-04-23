extern crate hyper;
extern crate url;
extern crate serde_json;
extern crate rand;

mod conf;
mod request;
mod work;

fn main() {
    let config = conf::read_config(&"config.ini".to_string());

    if let Some(token) = config.get("token") {
        if let Some(login) = config.get("root-login") {
            let options = conf::SearchOption {
                min_depth: config.get("depth-min"),
                max_depth: config.get("depth-max"),
                min_star: config.get("star-min"),
                max_star: config.get("star-max"),
                languages: config.get("languages"),
            };

            if let Some(all_repo) = work::search(token, login, &options, 0) {
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
