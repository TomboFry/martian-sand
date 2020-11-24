use crate::cell::Cell;
use crate::element::Element;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use rand::prelude::*;
use std::time::Instant;

pub struct World {
	// Cursor control
	mouse_x: usize,
	mouse_y: usize,
	cursor_radius: usize,

	pub elements: Vec<Element>,
	pub is_drawing: bool,
	pub is_paused: bool,
	pub selected_element: Option<Element>,
	cells: Vec<Cell>,

	// Misc
	last_render: Instant,
	render_time: usize,
}

impl World {
	pub fn new() -> World {
		// Random cells
		let mut rng = thread_rng();
		let cells = (0..1024)
			.into_iter()
			.map(|_| {
				let colour = [rng.gen(), rng.gen(), rng.gen()];
				let element = Element::new("Sand", colour);
				let x = rng.gen_range(0, SCREEN_WIDTH);
				let y = rng.gen_range(0, SCREEN_HEIGHT);

				Cell::new(element, x as usize, y as usize)
			})
			.collect();

		World {
			mouse_x: 0,
			mouse_y: 0,
			cursor_radius: 16,
			elements: vec![],
			is_drawing: false,
			is_paused: false,
			selected_element: None,
			cells: cells,
			last_render: Instant::now(),
			render_time: 0,
		}
	}
}
