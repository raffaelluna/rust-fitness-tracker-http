use repository::surrealdb_repo::DbFairing;
use rocket::figment::providers::{Format, Toml};
use rocket::figment::Figment;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use rocket::Config;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

pub mod repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct Exercise {
    exercise_name: String,
    sets: i32,
    repetitions: i32,
    load: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workout {
    workout_date: String,
    workout_type: String,
    targeted_muscles: String,
    exercises: Vec<Exercise>,
}

impl Exercise {
    pub fn total_workload(&self) -> i32 {
        self.sets * self.repetitions * self.load
    }
}

#[get("/")]
async fn index() -> RawHtml<&'static str> {
    RawHtml("<h1>Trolebus!</h1>")
}

#[post("/", data = "<workout>")]
async fn add_workout(workout: Json<Workout>) {
    let mut msg_string = format!(
        "Got a valid {} workout, done at {}.\nExercises:\n",
        workout.targeted_muscles, workout.workout_date
    );

    for exercise in &workout.exercises {
        let exercise_fmt = format!(
            "\t - {}: {}x{} with {} kgs.\n",
            exercise.exercise_name,
            exercise.sets,
            exercise.repetitions,
            exercise.load
        );
        msg_string.push_str(exercise_fmt.as_str())
    }

    println!("{}", msg_string);
}

#[launch]
fn rocket() -> _ {
    let config = Figment::from(Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Toml::file("App.toml").nested());

    rocket::custom(config)
        .mount("/", routes![index, add_workout])
        .attach(DbFairing)
}
