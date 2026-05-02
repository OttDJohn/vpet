use std::time::Duration;

use rand::prelude::*;

use serde::{Serialize, Deserialize};

use crate::now_s;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PetType {
	Squid
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
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

impl Pet {
	pub fn new(now: u64) -> Pet {
		Pet {
			t: PetType::Squid,
			born: now,
			food: 5,
			training: 5,
			metabolism: 80,
			discipline: 80,
			alive: true
		}
	}

        pub fn is_egg(&self) -> bool {
		return now_s() - self.born < Duration::from_mins(1).as_secs();
	}

	pub fn tick(&mut self, rng: &mut impl Rng) {
	        // Don't tick if we're still in an egg.
		if self.is_egg() {
		    return;
		}
		if rng.random_range(0..100) < (100 - self.metabolism) {
			self.food = std::cmp::max(self.food - 1, -5);
		}

		if rng.random_range(0..100) < (100 - self.discipline) {
			self.training = std::cmp::max(self.training - 1, -5);
		}

		self.alive = (self.training > -5) && (self.food > -5);
	}
}
