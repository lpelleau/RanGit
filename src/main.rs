extern crate hyper;
extern crate url;
extern crate serde_json;
extern crate rand;

mod conf;
mod request;
mod work;

fn main() {
    let mut config = conf::Config::new("config.ini");
    config.load();
    let a = config.token();

    if let Some(all_repo) = work::search(&a.to_string(), &config.login(), &config, 0) {
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
