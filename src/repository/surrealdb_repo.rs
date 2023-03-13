use crate::model::workout_model::{Exercise, Workout};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Rocket};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::sql::{Object, Value};
use surrealdb::{Datastore, Error, Response, Session};

pub trait Creatable: Into<Value> {}
pub trait Patchable: Into<Value> {}

#[derive(Clone)]
pub struct SurrealDBRepo {
    pub ds: Arc<Datastore>,
    pub ses: Session,
}

impl SurrealDBRepo {
    pub async fn init(
        namespace: &str,
        database: &str,
        datastore: &str,
    ) -> Result<Self, Error> {
        Ok(Self {
            ds: Arc::new(Datastore::new(datastore).await.unwrap()),
            ses: Session::for_kv().with_ns(namespace).with_db(database),
        })
    }

    pub async fn execute(
        &self,
        query: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Vec<Response>, crate::error::Error> {
        todo!()
    }

    pub async fn add_workout(
        &self,
        workout: Workout,
    ) -> Result<Object, crate::error::Error> {
        todo!()
    }

    pub async fn get_workout(
        &self,
        targeted_muscles: &str,
        done_at: &str,
    ) -> Result<Object, crate::error::Error> {
        todo!()
    }

    pub async fn get_all_workouts(
        &self,
    ) -> Result<Vec<Object>, crate::error::Error> {
        todo!()
    }
}

pub struct DbFairing;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct DbConfig {
    namespace: String,
    database: String,
    datastore: String,
}

#[rocket::async_trait]
impl Fairing for DbFairing {
    fn info(&self) -> Info {
        Info {
            name: "Database",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(
        &self,
        rocket: Rocket<Build>,
    ) -> Result<Rocket<Build>, Rocket<Build>> {
        let figment = rocket.figment().clone();
        let db_config: DbConfig =
            figment.select("database").extract().unwrap();

        let db = SurrealDBRepo::init(
            &db_config.namespace,
            &db_config.database,
            &db_config.datastore,
        )
        .await
        .unwrap();

        Ok(rocket.manage(db))
    }
}
