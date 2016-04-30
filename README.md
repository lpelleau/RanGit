# Random Git Repository
## Synopsis
RanGit enable you to search repositories from a specific user by looking at his starred repositories and go see his following's starred repositories.

## Motivation
Finding a hidden project on Github which could be THE new project you shouldn't miss.  
This is my first project in Rust language.
A simple project to understand the basics.   
*Don't look at the code... Ugly.*

## Installation
Install Rust environment [on the official site](https://www.rust-lang.org/downloads.html).  
type in a terminal `git clone https://github.com/lpelleau/RanGit.git && cd RanGit && cp config.ini.sample config.ini && cargo build`.  
Get a token with Github API OAuth 2.0 and place it in the config file.

## Usage
Run the project with `cargo run`.
You can change settings in *config.ini* file.
All options except *token* and *root-login* are optional.

Available options are:
* **depth-min**: integer >= 0
* **depth-max**: integer > 0
* **star-min**: integer >= 0
* **star-max**: integer > 0
* **languages**: <Lang1, Lang2, ...>* (not case-sensitive)

*/!\ Some language's repositories cannot be detected with Github API.*

## Todo
There are still a lot of things to do:
* introduce cache for the results (with TTL ?): database (SQL/NoQL) ? file ?
* add concurrency;
* integrate *[rust-ini](https://github.com/zonyitoo/rust-ini)* lib ?
* avoid getting same result several times;
* avoid getting root-login repositories in results;
* add *--verbose* argument to program showing advancement of the requests;
