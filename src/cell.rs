use crate::element::Element;

#[derive(Clone)]
pub struct Cell {
	pub element: Element,
	pub x: usize,
	pub y: usize,
	pub alive: bool,
	seed: u64,
}

impl Cell {
	pub fn new(element: Element, x: usize, y: usize, seed: u64) -> Cell {
		Cell {
			element,
			x,
			y,
			alive: true,
			seed,
		}
	}

	pub fn step(&mut self) {
		self.seed = self.seed.rotate_left(1);

		if self.seed & 0xff > 192 {
			self.alive = false;
		}
	}
}
