use api::workout_api::{add_workout, index};
use repository::surrealdb_repo::DbFairing;
use rocket::figment::providers::{Format, Toml};
use rocket::figment::Figment;

use rocket::Config;

#[macro_use]
extern crate rocket;

mod api;
mod error;
mod model;
mod prelude;
mod repository;
mod utils;

#[launch]
fn rocket() -> _ {
    let config = Figment::from(Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Toml::file("App.toml").nested());

    rocket::custom(config)
        .mount("/", routes![index, add_workout])
        .attach(DbFairing)
}
