use crate::model::workout_model::{Exercise, Workout};
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;

#[get("/")]
pub async fn index() -> RawHtml<&'static str> {
    RawHtml("<h1>Trolebus!</h1>")
}

#[post("/", data = "<workout>")]
pub async fn add_workout(workout: Json<Workout>) {
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
