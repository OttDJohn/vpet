use rand::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PetType {
	Squid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pet {
	pub t: PetType,
	pub born: u64,
	pub food: i8,
	pub metabolism: u8,
	pub training: i8,
	pub discipline: u8,
	// At first we want to just delete the entry when we die but we
	// might want to reincarnate later.
	pub alive: bool,
}

pub fn tick(rng: &mut impl Rng, p: &mut Pet) {
	if rng.random_range(0..100) < p.metabolism {
		p.food -= 1;
	}

	if rng.random_range(0..100) < p.discipline {
		p.training -= 1;
	}

	p.alive = (p.training > -5) && (p.food > -5);
}
