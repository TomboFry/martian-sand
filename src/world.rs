use crate::cell::Cell;
use crate::element::Element;
use crate::util::draw;
use crate::{GUI_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH};

use rand::prelude::*;
use std::time::Instant;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

type Pixels = pixels::Pixels<winit::window::Window>;

pub struct World {
	world_width: usize,
	world_height: usize,

	// Cursor control
	mouse_x: usize,
	mouse_y: usize,
	cursor_radius: usize,

	pub elements: Vec<Element>,
	pub is_drawing: bool,
	pub is_paused: bool,
	pub selected_element: Option<usize>,
	cells: Vec<Cell>,

	// Misc
	last_render: Instant,
	render_time: usize,
}

impl World {
	pub fn new() -> World {
		let world_height = SCREEN_HEIGHT - GUI_HEIGHT;

		// Random cells (temporary)
		let mut rng = thread_rng();
		let cells = (0..1024)
			.into_iter()
			.map(|_| {
				let colour = [rng.gen(), rng.gen(), rng.gen()];
				let element = Element::new("Sand", colour);
				let x = rng.gen_range(1, SCREEN_WIDTH - 1);
				let y = rng.gen_range(1, world_height);

				Cell::new(element, x as usize, y as usize)
			})
			.collect();

		let colour = [rng.gen(), rng.gen(), rng.gen()];
		let element = Element::new("Potato", colour);

		World {
			world_width: SCREEN_WIDTH as usize,
			world_height: world_height as usize,
			mouse_x: 0,
			mouse_y: 0,
			cursor_radius: 16,
			elements: vec![element],
			is_drawing: false,
			is_paused: false,
			selected_element: Some(0),
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

		self.set_scroll(input);
		self.set_mouse_position(pixels, input);
		self.set_render_time();
		self.remove_out_of_bounds();

		if self.is_paused {
			return;
		}

		// Update Grid
	}

	fn set_scroll(&mut self, input: &WinitInputHelper) {
		let scroll = input.scroll_diff();
		self.cursor_radius = (self.cursor_radius as f32 + (scroll * 2.0)) as usize;
	}

	fn set_mouse_position(&mut self, pixels: &Pixels, input: &WinitInputHelper) {
		if input.mouse_pressed(0) {
			self.is_drawing = true;
		}
		if input.mouse_released(0) {
			self.is_drawing = false;
		}

		input.mouse()
			.map(|(mx, my)| {
				let (mx_p, my_p) = pixels
					.window_pos_to_pixel((mx, my))
					.unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

				self.mouse_x = mx_p;
				self.mouse_y = my_p;
			})
			.unwrap_or_default();
	}

	fn set_render_time(&mut self) {
		let now = Instant::now();
		self.render_time = now.duration_since(self.last_render).as_millis() as usize;
		self.last_render = Instant::now();
	}

	fn remove_out_of_bounds(&mut self) {
		let height = self.world_height;
		let width = self.world_width;
		self.cells
			.retain(|cell| cell.x > 0 && cell.x < width - 1 && cell.y > 0 && cell.y < height);
	}

	pub fn draw(&mut self, screen: &mut [u8]) {
		// Draw each cell to screen
		self.cells.iter().for_each(|cell: &Cell| {
			draw::pixel(screen, cell.x, cell.y, cell.element.colour);
		});

		// Draw cursor
		if let Some(elm_index) = self.selected_element {
			let element = &self.elements[elm_index];
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
