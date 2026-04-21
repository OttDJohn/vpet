use std::fs::DirBuilder;
use std::path;

use const_format::concatcp;

use heed::{Database, Env, EnvOpenOptions};
use heed::types::*;

use ohkami::{Ohkami, Route};
use ohkami::claw::{Json, Path};
use ohkami::fang::Context;
use ohkami::serde::{Deserialize, Serialize};

use uuid::Uuid;

const API_PREFIX: &'static str = "/api/v1/";

#[derive(Serialize)]
struct PetResult {
	id: Uuid,
}

async fn create_pet(Context(dbenv): Context<'_, Env>) -> Json<PetResult> {
	let u = Uuid::new_v4();
	let mut wtx = dbenv.write_txn().expect("Failed to open write transaction");
	let pets: Database<Str, Str> = dbenv.create_database(&mut wtx, Some("pets"))
		.expect("Couldn't open pets db");
	pets.put(&mut wtx, &u.to_string(), "exists");
	wtx.commit().expect("Failed to commit new pet");

	Json(PetResult { id: u })
}

#[tokio::main]
async fn main() {
	let path = path::Path::new("./petdb");
	DirBuilder::new().recursive(true).create(path).expect("Unable to create db path");
	let env = unsafe { EnvOpenOptions::new().max_dbs(8).open(path).expect("Failed to open db") };

	Ohkami::new((Context::new(env),
		concatcp!(API_PREFIX, "pet").POST(create_pet))).howl("localhost:8080").await
}
