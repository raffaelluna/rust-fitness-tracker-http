use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Object, Value};

use crate::prelude::*;
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};
use crate::utils::macros::map;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exercise {
    pub exercise_name: String,
    pub sets: i32,
    pub repetitions: i32,
    pub load: i32,
}

impl Exercise {
    pub fn total_workload(&self) -> i32 {
        self.sets * self.repetitions * self.load
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workout {
    pub workout_date: String,
    pub workout_type: String,
    pub targeted_muscles: String,
    pub exercises: Vec<Exercise>,
}

impl Workout {
    fn generate_workout_uuid(&self) -> String {
        let uuid_str =
            format!("{}{}", &self.workout_date, &self.targeted_muscles);

        Uuid::parse_str(uuid_str.as_str())
            .as_ref()
            .unwrap()
            .to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ExerciseDone {
    workout_uuid: String,
    order_no: usize,
    exercise: Exercise,
    total_workload: i32,
    done_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExercisesDone {
    exercises_done: Vec<ExerciseDone>,
}

impl From<Workout> for ExercisesDone {
    fn from(workout: Workout) -> Self {
        let exercises_done = workout
            .exercises
            .iter()
            .enumerate()
            .map(|(order_no, ex)| ExerciseDone {
                workout_uuid: workout.generate_workout_uuid(),
                order_no: order_no,
                exercise: ex.to_owned(),
                total_workload: ex.total_workload(),
                done_at: workout.workout_date.clone(),
            })
            .collect::<Vec<_>>();

        Self { exercises_done }
    }
}

fn test(workout: Workout) {
    let exercises_done = ExercisesDone::from(workout);

    for exercise_done in exercises_done.exercises_done {
        println!("{:?}", exercise_done);
    }
}
