#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;

mod utils;

#[get("/")]
fn index() -> &'static str {
  return  "Welcome to Hopper!";
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let _command = utils::get_command_from_query_string(&cmd);
    let _redirect_url = match _command {
        "tw"=> utils::twitter::construct_twitter_url(&cmd),
        "gh"=> utils::github::construct_github_url(&cmd),
        "st"=> utils::stackoverflow::construct_stackoverflow_url(&cmd),
        "rd"=> utils::reddit::construct_reddit_url(&cmd),
        "ln"=> utils::linkedin::construct_linkedin_url(&cmd),
        _=> utils::google::construct_google_search_from_query(&cmd)
    };
    return Redirect::to(_redirect_url);
}

fn main() {
   rocket::ignite().mount("/", routes![index, search]).launch();
}
