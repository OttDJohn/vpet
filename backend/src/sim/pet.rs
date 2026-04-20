use rand::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Pet {
	age: u8,
	food: i8,
	metab: u8,
	fun: i8,
	atten: u8,
	// At first we want to just delete the entry when we die but we
	// might want to reincarnate later.
	alive: bool,
}

fn tick(rng: &Rng, p: &mut Pet) {
	if (rng::random_range(0..100) < p.metab) {
		p.food--;
	}

	if (rng::random_range(0..100) < p.atten) {
		p.fun--;
	}

	p.alive = (p.fun > -5) && (p.food > -5);
}
