use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Object, Value};

use crate::prelude::*;
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};
use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Exercise {
    pub exercise_name: String,
    pub sets: i32,
    pub repetitions: i32,
    pub load: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workout {
    pub workout_date: String,
    pub workout_type: String,
    pub targeted_muscles: String,
    pub exercises: Vec<Exercise>,
}

impl Exercise {
    pub fn total_workload(&self) -> i32 {
        self.sets * self.repetitions * self.load
    }
}
