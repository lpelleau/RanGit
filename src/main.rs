extern crate hyper;
extern crate url;
extern crate serde_json;
extern crate rand;

mod conf;
mod request;
mod work;

use std::env;

fn main() {
    let mut verbose = false;
    if let Some(arg1) = env::args().nth(1) {
        if arg1 == "-v" || arg1 == "--verbose" {
            verbose = true;
        }
    }

    let mut config = conf::Config::new("config.ini".to_string());
    if let Err(err) = config.load() {
        panic!("{}", err);
    }

    let mut api = request::APIRest::new("https://api.github.com/users/".to_string());
    api.set_user_agent("User-Agent: lpelleau/RanGit".to_string());

    let login = &config.login().clone();
    let mut search = work::Search::new(config, api);
    let all_repo = search.compute(login, verbose);

    if all_repo.len() > 0 {
        let selected = rand::random::<usize>() % all_repo.len();

        if let Some(repo) = all_repo.get(selected) {
            println!("Found: ");
            for i in 0..(repo.len() - 1) {
                print!("{} -> ", repo[i]);
            }
            println!("\n\t= {}", repo[repo.len() - 1]);
        }
    } else {
        println!("No repository found with your criteria.")
    }
}
