mod sim;

use sim::pet::{Pet, PetType};
use std::fs::DirBuilder;
use std::sync::{Arc, LazyLock};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use const_format::concatcp;

use heed::{Database, Env, EnvOpenOptions};
use heed::types::*;

use ohkami::prelude::*;
use ohkami::{Ohkami, Route};
use ohkami::claw::{Json, Path};
use ohkami::fang::Context;
use ohkami::serde::{Serialize};

use serde_json;

use uuid::Uuid;

const API_PREFIX: &'static str = "/api/v1/";

#[derive(Serialize)]
struct CreatePetResult {
	id: Uuid,
}


#[derive(Serialize)]
struct WirePet {
	t: PetType,
	food: i8,
	training: i8,
	age: u64
}

#[derive(Serialize)]
struct Error {
    status_code: u16,
    message:     String,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		Response::new(Status::from(self.status_code)).with_json(self)
	}
}
impl From<uuid::Error> for Error {
	fn from(_e: uuid::Error) -> Error {
		Error { status_code: 400, message: String::from("Invalid uuid") }
	}
}

fn wire_of_pet(p: Pet) -> WirePet {
	const DAYS_OF_SECONDS: u64 = 60 * 60 * 24;
	WirePet {t: p.t, food: p.food, training: p.training, age: (now_s() - p.born) / (DAYS_OF_SECONDS) }
}

async fn create_pet(Context(dbenv): Context<'_, Arc<Env>>) -> Json<CreatePetResult> {
	let u = Uuid::new_v4();
	let mut wtx = dbenv.write_txn().expect("Failed to open write transaction");
	let pets: Database<Str, Str> = dbenv.create_database(&mut wtx, Some("pets"))
		.expect("Couldn't open pets db");
	_ = pets.put(&mut wtx, &u.to_string(), &(serde_json::to_string(&Pet::new(now_s()))
	    	.unwrap()));
	wtx.commit().expect("Failed to commit new pet");

	Json(CreatePetResult { id: u })
}

async fn get_pet(Context(dbenv): Context<'_, Arc<Env>>, Path(id): Path<&str>) -> Result<Json<WirePet>, Error> {
	let _u = Uuid::parse_str(id)?; // Make sure it's a valid uuid

	let mut rtx = dbenv.read_txn().expect("Failed to open read transaction");
	let pets: Database<Str, Str> = dbenv.open_database(&mut rtx, Some("pets")).or_else(|_x| Err(Error {status_code: 404, message: String::from("Pet not found!")}))?.expect("Broken database when calling get_pet");
	if let Ok(pet) = pets.get(&rtx, id) {
		match pet {
			Some(pet) => Ok(Json(wire_of_pet(serde_json::from_str(pet).unwrap()))),
			None => Err(Error {status_code: 404, message: String::from("Invalid id")})
		}
	} else {
		Err(Error {status_code: 500, message: String::from("Error getting from db")})
	}
}

fn update_pet<F>(env: &Env, id: &str, cb: F) -> Result<Pet, Error> 
	where F: Fn(&mut Pet) {
	let mut wtx = env.write_txn().expect("Failed to open write transaction");
	let pets: Database<Str, Str> = env.create_database(&mut wtx, Some("pets"))
		.expect("Couldn't open pets db");
	if let Ok(pet) = pets.get(&wtx, id) {
		if let Some(ps)  = pet {
			let mut p: Pet = serde_json::from_str(ps).unwrap();
			cb(&mut p);
			_ = pets.put(&mut wtx, id, &(serde_json::to_string(&p).unwrap()));
			wtx.commit().expect("Failed to commit while updating pet");
			Ok(p)
		} else {
			Err(Error { status_code: 404, message: String::from("Pet not found")})
		}
	} else {
		Err(Error { status_code: 500, message: String::from("Failed to get from database")})
	}
}

async fn train_pet(Context(dbenv): Context<'_, Arc<Env>>, Path(id): Path<&str>) -> Result<Json<WirePet>, Error> {
	let u = Uuid::parse_str(id)?; // Make sure it's a valid uuid
	update_pet(&(*dbenv), id, | p | { p.food = std::cmp::min(5, p.training + 1) }).map(|p| { Json(wire_of_pet(p))})
}

async fn feed_pet(Context(dbenv): Context<'_, Arc<Env>>, Path(id): Path<&str>) -> Result<Json<WirePet>, Error> {
	let u = Uuid::parse_str(id)?; // Make sure it's a valid uuid
	update_pet(&(*dbenv), id, | p | { p.food = std::cmp::min(5, p.food + 1) }).map(|p| { Json(wire_of_pet(p))})
}

fn now_s() -> u64 {
	SystemTime::now().duration_since(UNIX_EPOCH).expect("Something bad happened with time").as_secs()
}

fn tick_pet_thread(dbenv: Arc<Env>) {
	loop {
		eprintln!("Tick!");
		let mut rng = rand::rng();
		let mut wtx = (*dbenv).write_txn().expect("Failed to open pet tick transaction");
		if let Ok(pets) = dbenv.create_database::<Str,Str>(&mut wtx, Some("pets")) {
			let iter = pets.iter_mut(&mut wtx);
			if let Ok(mut ri) = iter {
				while let Some(val) = ri.next() {
					let v: (&str, &str) = val.unwrap();
					let rv = v.1;
					let k = v.0;
					let mut p: Pet = serde_json::from_str(rv).unwrap();
					p.tick(&mut rng);
					unsafe {_ = ri.put_current(&k, &serde_json::to_string(&p).unwrap())};
				}
			}
		} else {
			panic!("Failed to open database for ticks");
		}
		wtx.commit().expect("Failed to commit pet ticks");
		thread::sleep(Duration::from_secs(5));
	}
}
	
#[tokio::main]
async fn main() {
	DirBuilder::new().recursive(true).create("./petdb").expect("Unable to create db path");
	/* Yes this is fucking horrific. Blame Rust.
	 * To explain: we need the Env available on both the REST thread (to create, feed, etc.) and on the sim thread.
	 * We can't share data between the threads because the Env isn't cloneable. We can't *not* share the data because
	 * only one env is valid in the program. So we need to wrap it in an Automated Reference Count container (Arc).
	 * The problem then is that Rust can't prove that the sim thread (running tick_pet_thread) will end before the main thread
	 * which means that it has to be static. But if it's static then every call in initialization must be const (can be run at
	 * compile time) which in this case it's not. So we have to wrap the Arc in a LazyLock, which isn't a lock, it's just lazy
	 * evaluation. Meaning that all we create at runtime is the "lock" with a const-friendly lambda, which can be static.
	 */
	static ENV: LazyLock<Arc<Env>> = unsafe { std::sync::LazyLock::new(|| {Arc::new(EnvOpenOptions::new().max_dbs(8).open("./petdb").expect("Failed to open db"))}) };
	thread::spawn(|| {tick_pet_thread((*ENV).clone())});
	Ohkami::new((Context::new((*ENV).clone()),
		concatcp!(API_PREFIX, "pet").POST(create_pet),
		concatcp!(API_PREFIX, "pet/:id").GET(get_pet),
		concatcp!(API_PREFIX, "pet/:id/feed").POST(feed_pet),
		concatcp!(API_PREFIX, "pet/:id/train").POST(feed_pet),
		)).howl("localhost:8080").await
}
