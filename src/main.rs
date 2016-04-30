extern crate hyper;
extern crate url;
extern crate serde_json;
extern crate rand;

mod conf;
mod request;
mod work;

fn main() {
    let mut config = conf::Config::new("config.ini".to_string());
    {
        match config.load() {
            Err(err) => panic!("{}", err),
            _ => ()
        };
    }

    let mut api = request::APIRest::new("https://api.github.com/users/".to_string());
    api.set_user_agent("User-Agent: lpelleau/RanGit".to_string());

    let login = &config.login().clone();
    let search = work::Search::new(config, api);
    let all_repo = search.compute(login);

    if all_repo.len() > 0 {
        let selected = rand::random::<usize>() % all_repo.len();

        if let Some(repo) = all_repo.get(selected) {
            println!("Found: {}", repo);
        }
    } else {
        println!("No repository found with your criteria.")
    }
}
