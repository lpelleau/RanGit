# Random Git Repository
## Synopsis
RanGit enable you to search repositories from a specific user by looking at his starred repositories and go see his following's starred repositories.

## Motivation
Finding a hidden project on Github which could be THE new project you shouldn't miss.  
This is my first project in Rust langage. 
A simple project to understand the basics.   
*Don't look at the code.*

## Installation
Install Rust environment [on the official site](https://www.rust-lang.org/downloads.html).  
type in a terminal `git clone https://github.com/lpelleau/RanGit.git && cd RanGit && cp config.ini.sample config.ini && cargo build`.  
Get a token with Github API OAuth 2.0 and place it in the config file.

## Usage
Run the project with `cargo run`.

## Todo
There are still a lot of things to do:
* get root user (for search) with option;
* add parameters:
  * MIN and MAX depth in following;
  * langage selection (severals);
  * MIN and MAX stars on repository;
* Restructuration of the code (with library for JSON requests ?)
