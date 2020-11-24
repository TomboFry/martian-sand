use crate::cell::Cell;
use crate::element::Element;
use crate::util::draw;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use rand::prelude::*;
use std::time::Instant;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

type Pixels = pixels::Pixels<winit::window::Window>;

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

	pub fn pause_toggle(&mut self) {
		self.is_paused = !self.is_paused;
	}

	pub fn update(&mut self, pixels: &Pixels, input: &WinitInputHelper) {
		if input.key_pressed(VirtualKeyCode::P) {
			self.pause_toggle();
		}

		if self.is_paused {
			return;
		}

		let scroll = input.scroll_diff();
		self.cursor_radius = (self.cursor_radius as f32 + (scroll * 2.0)) as usize;

		input.mouse()
			.map(|(mx, my)| {
				let (mx_p, my_p) = pixels
					.window_pos_to_pixel((mx, my))
					.unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

				self.mouse_x = mx_p;
				self.mouse_y = my_p;
			})
			.unwrap_or_default();

		// Update Grid

		// Set render stats
		let now = Instant::now();
		self.render_time = now.duration_since(self.last_render).as_millis() as usize;
		self.last_render = Instant::now();
	}

	pub fn draw(&mut self, screen: &mut [u8]) {
		// Draw each cell to screen
		self.cells.iter().for_each(|cell: &Cell| {
			draw::pixel(screen, cell.x, cell.y, cell.element.colour);
		});

		// Draw cursor
		if let Some(element) = &self.selected_element {
			draw::circle(screen, self.mouse_x, self.mouse_y, self.cursor_radius, element.colour);
			draw::text(
				screen,
				self.mouse_x + self.cursor_radius,
				self.mouse_y + self.cursor_radius,
				&element.name,
			);
		}

		draw::text(screen, 16, 16, &format!("{} ms", self.render_time));
	}
}
