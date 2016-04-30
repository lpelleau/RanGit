extern crate hyper;
extern crate url;
extern crate serde_json;
extern crate rand;

mod conf;
mod request;
mod work;

fn main() {
    let mut config = conf::Config::new("config.ini");
    {
        let load_res = config.load();
        if load_res.is_err() {
            panic!("{}", load_res.err().unwrap());
        }
    }

    let mut api = request::APIRest::new("https://api.github.com/users/".to_string());
    api.set_user_agent("User-Agent: lpelleau/RanGit".to_string());

    if let Some(all_repo) = work::search(&config, &api, &config.login(), 0) {
        if all_repo.len() > 0 {
            let selected = rand::random::<usize>() % all_repo.len();

            if let Some(repo) = all_repo.get(selected) {
                println!("Found: {}", repo);
            }
        } else {
            println!("No repository found with your criteria.")
        }
    }
}
