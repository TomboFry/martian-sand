use crate::cell::Cell;
use crate::element::Element;
use crate::util::circle::circle_collision;
use crate::util::draw;
use crate::{GUI_HEIGHT, RGB, SCREEN_HEIGHT, SCREEN_WIDTH};

use rand::prelude::*;
use rayon::prelude::*;

use std::time::Instant;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub struct World {
	world_width: usize,
	world_height: usize,

	// Cursor control
	mouse_x: usize,
	mouse_y: usize,
	mouse_down_left: bool,
	mouse_down_right: bool,
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

		let colour: RGB = [rng.gen(), rng.gen(), rng.gen()];
		let element = Element::new("Potato", colour);

		World {
			world_width: SCREEN_WIDTH as usize,
			world_height: world_height as usize,
			mouse_x: 0,
			mouse_y: 0,
			mouse_down_left: false,
			mouse_down_right: false,

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

	pub fn update(&mut self, input: &WinitInputHelper) {
		if input.key_pressed(VirtualKeyCode::P) {
			self.pause_toggle();
		}

		self.set_scroll(input);
		self.set_mouse_position(input);
		self.set_render_time();
		self.add_element();
		self.remove_out_of_bounds();

		if self.is_paused {
			return;
		}

		// Update Grid
		self.cells.iter_mut().for_each(|cell| cell.step());
	}

	fn set_scroll(&mut self, input: &WinitInputHelper) {
		let scroll = input.scroll_diff();
		self.cursor_radius = (self.cursor_radius as f32 + (scroll * 2.0)) as usize;
	}

	fn set_mouse_position(&mut self, input: &WinitInputHelper) {
		// Set left click
		if input.mouse_pressed(0) {
			self.mouse_down_left = true;
		}
		if input.mouse_released(0) {
			self.mouse_down_left = false;
		}

		// Set right click
		if input.mouse_pressed(1) {
			self.mouse_down_right = true;
		}
		if input.mouse_released(1) {
			self.mouse_down_right = false;
		}

		input
			.mouse()
			.map(|(mx, my)| {
				self.mouse_x = mx.clamp(0.0, SCREEN_WIDTH as f32) as usize;
				self.mouse_y = my.clamp(0.0, SCREEN_HEIGHT as f32) as usize;
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
			.retain(|cell| cell.x > 0 && cell.x < width - 1 && cell.y > 0 && cell.y < height && cell.alive);
	}

	fn add_element(&mut self) {
		self.is_drawing = false;
		if self.mouse_down_left {
			self.is_drawing = true;
		}

		if self.is_drawing == false || self.selected_element.is_none() {
			return;
		}

		let elm_index = self.selected_element.unwrap().clone();
		let mx = self.mouse_x;
		let my = self.mouse_y;
		let rad = self.cursor_radius;

		let x_lo = if rad > mx { 0 } else { mx - rad };
		let x_hi = mx + rad;
		let y_lo = if rad > my { 0 } else { my - rad };
		let y_hi = my + rad;

		let cells_in_box = self.cells_in_box(x_lo, y_lo, x_hi, y_hi).clone();
		for px in x_lo..x_hi {
			for py in y_lo..y_hi {
				if !circle_collision(px, py, mx, my, self.cursor_radius, 0.0) {
					continue;
				}

				let collision = cells_in_box.iter().find(|(x, y)| *x == px && *y == py).is_some();
				if collision {
					continue;
				}

				let element = self.elements[elm_index].clone();
				self.cells.push(Cell::new(element, px, py))
			}
		}
	}

	fn cells_in_box(&mut self, x: usize, y: usize, x2: usize, y2: usize) -> Vec<(usize, usize)> {
		self.cells
			.iter()
			.filter(|cell| cell.x > x && cell.y > y && cell.x < x2 && cell.y < y2)
			.map(|cell| (cell.x, cell.y))
			.collect()
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

		draw::rect(screen, 8, 8, 108, 56, [0x33, 0x33, 0x33]);
		draw::text(screen, 16, 16, &format!("{} ms", self.render_time));
		draw::text(screen, 16, 24, &format!("Cells: {}", self.cells.len()));
		draw::text(screen, 16, 32, &format!("Paused: {}", self.is_paused));
		draw::text(screen, 16, 40, &format!("Drawing: {}", self.is_drawing));
	}
}
