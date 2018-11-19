#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;
extern crate rocket;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate failure;

use dotenv::dotenv;

mod database;
mod backend;
mod middleware;

fn main() {
    dotenv().ok();
    backend::routes::create_routes();
}