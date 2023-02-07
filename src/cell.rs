// use crate::qtree::QuadTree;
use crate::{element::Element, interaction::Interaction, qtree::QuadTree, RGB};

const GRAVITY: f32 = 3.0;

#[derive(Clone)]
pub struct Cell {
	// pub element: Element,
	pub x: f32,
	pub y: f32,
	pub x_r: usize,
	pub y_r: usize,
	pub x_prev: usize,
	pub y_prev: usize,
	pub alive: bool,
	seed: u64,
	pub duration: usize,
	pub max_duration: Option<usize>,

	// Copy element properties
	pub elm_index: usize,
	pub colour: RGB,
	density: f32,
	viscosity: f32,
	temperature: f32, // 0 - 100
	temperature_delta: f32,
	pub interactions: Vec<Interaction>,
	pub emit_element: Option<usize>,
	pub emit_chance: f32,
}

fn round(val: f32) -> usize {
	val.round() as usize
}

impl Cell {
	pub fn new(element: &Element, elm_index: usize, x: usize, y: usize, seed: u64) -> Cell {
		Cell {
			x: x as f32,
			y: y as f32,
			x_r: x,
			y_r: y,
			x_prev: x,
			y_prev: y,
			alive: true,
			seed,
			duration: (seed >> 16) as usize & 0x3f,

			elm_index,
			max_duration: element.max_duration,
			colour: element.colour,
			density: element.density,
			viscosity: element.viscosity,
			temperature: element.temperature,
			temperature_delta: element.temperature_delta,
			interactions: element.interactions.clone(),
			emit_element: element.emit_element,
			emit_chance: element.emit_chance,
		}
	}

	fn emit(&mut self, emit: &mut Vec<Cell>, elements: &[Element]) {
		if let Some(elm_index) = self.emit_element {
			let x = self.x_r + self.random(4) as usize - 2;
			let y = self.y_r + self.random(4) as usize - 2;
			let cell = Cell::new(&elements[elm_index], elm_index, x, y, self.random(u64::MAX));
			emit.push(cell);
		}
	}

	fn kill(&mut self, emit: &mut Vec<Cell>, elements: &[Element], index: usize) -> (usize, Option<usize>) {
		self.alive = false;
		if self.emit_chance == 0.0 {
			self.emit(emit, elements);
		}
		(index, None)
	}

	pub fn change_element(&mut self, element: &Element, elm_index: usize) {
		self.alive = true;
		self.elm_index = elm_index;
		self.duration = 0;
		self.max_duration = element.max_duration;
		self.colour = element.colour;
		self.density = element.density;
		self.viscosity = element.viscosity;
		self.temperature = element.temperature;
		self.interactions = element.interactions.clone();
		self.emit_element = element.emit_element;
		self.emit_chance = element.emit_chance;
	}

	fn random(&mut self, max: u64) -> u64 {
		self.seed = (214013 * self.seed) + 2531011;
		(self.seed >> 16) & max
	}

	fn check_random(&mut self, index: usize, tree: &mut QuadTree<usize>) -> (usize, Option<usize>) {
		let check_x = self.x_r + self.random(2) as usize - 1;
		let check_y = self.y_r + self.random(2) as usize - 1;
		(index, tree.get(check_x, check_y))
	}

	pub fn step(
		&mut self,
		index: usize,
		tree: &mut QuadTree<usize>,
		emit: &mut Vec<Cell>,
		elements: &[Element],
		width: usize,
		height: usize,
	) -> (usize, Option<usize>) {
		self.duration += 1;

		self.x_prev = self.x_r;
		self.y_prev = self.y_r;

		if let Some(max_duration) = self.max_duration {
			if self.duration > max_duration && self.seed & 0xff > 192 {
				return self.kill(emit, elements, index);
			}
		}

		self.temperature += self.temperature_delta;

		if self.temperature <= 0.0 {
			self.temperature = 0.0;
			return self.check_random(index, tree);
		}

		if self.emit_chance > 0.0 {
			let chance = self.random(1000) as f32 / 1000.0;
			if chance < self.emit_chance {
				self.emit(emit, elements);
			}
		}

		let r_pve = (self.random(100) as f32 - 20.0) / 80.0;
		let r_pve_b = self.random(20) as f32 - 10.0;
		let r_scale = self.random(100) as f32 / 25.0;
		let temperature_scale = self.temperature * 0.01 * r_scale;

		let mut y_move = GRAVITY * self.density * (1.0 - self.viscosity) * (temperature_scale * r_pve);
		let mut x_move = (1.0 - self.density) * (1.0 - self.viscosity) * (temperature_scale / 4.0) * r_pve_b;

		if y_move == 0.0 && x_move == 0.0 {
			return self.check_random(index, tree);
		}

		let mut x_new = self.x + x_move;
		let mut y_new = self.y + y_move;
		let mut x_r_new = round(x_new);
		let mut y_r_new = round(y_new);

		if tree.get(x_r_new, y_r_new).is_some() {
			y_move /= 2.0;
			x_move *= 2.0;
			x_new = self.x + x_move;
			y_new = self.y + y_move;
			x_r_new = round(x_new);
			y_r_new = round(y_new);
			let collision = tree.get(x_r_new, y_r_new);
			if collision.is_some() {
				return (index, collision);
			}
		}

		self.x = x_new;
		self.y = y_new;
		self.x_r = x_r_new;
		self.y_r = y_r_new;

		if x_r_new == self.x_r || y_r_new == self.y_r {
			return (index, None);
		}

		tree.remove(self.x_prev, self.y_prev).unwrap();
		if self.x_r >= width || self.y_r >= height {
			return self.kill(emit, elements, index);
		}
		tree.insert(self.x_r, self.y_r, index).unwrap();

		(index, None)
	}
}
