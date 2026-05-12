use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use serde::Deserialize;
use crate::database::setup::database_setup::{database_get_setup_toggle, database_get_setup_volume};
use crate::global::global::PREFIX_FOR_SETUP;
use crate::inits::setup::init_tables_setup::init_tables;
use crate::types::setup::type_setup::Setup;

const RESOURCES: &str = include_str!("../../ressources/setup.json");

#[derive(Deserialize)]
struct Config {
    id: String
}

fn make_setup(id: &str) -> Setup {

    let setup_id = format!("{}_{}", PREFIX_FOR_SETUP, id);

    Setup {
        setup_id: setup_id.clone(),
        toggle: Arc::new(AtomicBool::new(database_get_setup_toggle(&setup_id))),
        volume: Arc::new(Mutex::new(database_get_setup_volume(&setup_id.clone())))
    }
}

pub fn init() -> Vec<Setup> {

    {
        init_tables()
    }

    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    config
        .iter()
        .map(|setup| make_setup(&setup.id))
        .collect()
}