use crate::element::Element;
use rand::prelude::*;

pub struct Cell {
	pub element: Element,
	pub x: usize,
	pub y: usize,
	pub alive: bool,
	rng: ThreadRng,
}

impl Cell {
	pub fn new(element: Element, x: usize, y: usize) -> Cell {
		Cell {
			element,
			x,
			y,
			alive: true,
			rng: thread_rng(),
		}
	}

	pub fn step(&mut self) {
		let rng = self.rng.gen::<u8>();
		if rng > 250 {
			self.alive = false;
		}
	}
}
