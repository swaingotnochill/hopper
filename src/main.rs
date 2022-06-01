#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::Redirect;

mod utils;

#[get("/")]
fn index() -> &'static str {
   "Hello, World!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let _command = utils::get_command_from_query_string(&cmd);
    let _redirect_url = match _command {
        "tw"=> utils::twitter::construct_twitter_url(&cmd),
        _=> utils::google::construct_google_search_from_query(&cmd)
    };
    return Redirect::to(_redirect_url);
}

fn main() {
   rocket::ignite().mount("/", routes![index, search]).launch();
}
