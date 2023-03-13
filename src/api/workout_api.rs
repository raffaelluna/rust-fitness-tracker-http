use crate::model::workout_model::Workout;
use crate::repository::surrealdb_repo::SurrealDBRepo;
use rocket::serde::json::Json;
use rocket::State;
use surrealdb::sql::Object;

#[get("/workout/<targeted_muscles>/<done_at>")]
pub async fn get_workout(
    targeted_muscles: &str,
    done_at: &str,
    db: &State<SurrealDBRepo>,
) -> Result<Json<Object>, std::io::Error> {
    todo!()
}

#[get("/workouts")]
pub async fn get_all_workouts(
    db: &State<SurrealDBRepo>,
) -> Result<Json<Vec<Object>>, std::io::Error> {
    todo!()
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
