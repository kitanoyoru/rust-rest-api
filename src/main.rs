use std::env;

#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations

extern crate actix_rt;
extern crate dotenv;
extern crate env_logger;

mod config;

#[actix_rt::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST .env var was not specified");
    let app_port = env::var("APP_PORT").expect("APP_PORT .env var was not specified");
    let app_url = env::var("APP_URL").expect("APP_URL .env var was not specified");
    let db_url = env::var("DB_URL").expect("DB_URL .env var was not specified");

    let db_pool = config::db::migrate_and_configure_db(&db_url);
}
