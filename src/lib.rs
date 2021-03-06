#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel_derive_enum;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::config::{Config, Environment, Value};
use rocket::fairing::AdHoc;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};
use std::collections::HashMap;

mod db;
mod errors;
mod models;
mod response;
mod routes;
#[allow(unused_imports)]
mod schema;

fn cors_fairing() -> Cors {
    rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Put,
            rocket::http::Method::Delete,
            rocket::http::Method::Head,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Cors fairing cannot be created")
}

embed_migrations!("migrations");

fn run_db_migrations(rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> {
    let conn = db::Conn::get_one(&rocket).expect("Cannot get db connection");
    match embedded_migrations::run(&*conn) {
        Ok(_) => Ok(rocket),
        Err(_) => Err(rocket),
    }
}

pub fn rocket(port: u16, db: &str) -> rocket::Rocket {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from(db));
    databases.insert("pg", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .attach(db::Conn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .attach(cors_fairing())
        .mount("/", StaticFiles::from("dist"))
        .mount(
            "/api/",
            routes![
                routes::users::get_current_user,
                routes::users::get_user_by_id,
                routes::users::post_users,
                routes::users::post_users_login,
                routes::users::post_users_logout,
                routes::products::get_products,
                routes::products::get_product_by_id,
                routes::products::post_products,
                routes::cart::get,
                routes::cart::update,
                routes::cart::clear,
                routes::orders::get,
                routes::orders::list,
                routes::orders::create,
                routes::orders::pick,
                routes::orders::update_status,
            ],
        )
}
