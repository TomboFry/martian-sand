use crate::cell::Cell;
use crate::element::Element;
use crate::qtree::QuadTree;
use crate::util::circle::circle_collision;
use crate::util::draw;
use crate::{GUI_WIDTH, SCREEN_HEIGHT, SCREEN_SCALE, SCREEN_WIDTH};

use rand::prelude::*;
// use rayon::prelude::*;

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

	pub is_drawing: bool,
	pub is_paused: bool,

	pub elements: Vec<Element>,
	pub selected_element: usize,

	cells: Vec<Cell>,
	tree: QuadTree<usize>,

	// Misc
	last_render: Instant,
	render_time: f32,
	rng: ThreadRng,
}

impl World {
	pub fn new() -> World {
		let world_width = (SCREEN_WIDTH - GUI_WIDTH) as usize;
		let world_height = SCREEN_HEIGHT as usize;
		let tree = QuadTree::new(0, 0, world_width, world_height, 500);

		let mut rng = thread_rng();
		let mut elements = Element::from_file();

		if elements.len() == 0 {
			elements.push(Element::random(0, &mut rng));
		}

		World {
			world_width,
			world_height,

			mouse_x: 0,
			mouse_y: 0,
			mouse_down_left: false,
			mouse_down_right: false,

			cursor_radius: 16,
			is_drawing: false,
			is_paused: false,

			elements,
			selected_element: 0,

			cells: Vec::with_capacity(world_width * world_height),
			tree,

			last_render: Instant::now(),
			render_time: 1.0,
			rng,
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

		self.remove_out_of_bounds();
		self.cells.sort_by(|a, b| a.duration.partial_cmp(&b.duration).unwrap());
		self.create_tree();
		self.add_element();

		if self.is_paused {
			return;
		}

		// Update Grid
		let tree = &mut self.tree;
		let elements = &self.elements;
		let width = self.world_width;
		let height = self.world_height;
		let mut emit: Vec<Cell> = vec![];

		let collisions = self
			.cells
			.iter_mut()
			.enumerate()
			.filter_map(|(index, cell)| {
				let collision = cell.step(index, tree, &mut emit, elements, width, height);
				if let Some(index) = collision.1 {
					return Some((collision.0, index));
				}
				None
			})
			.collect::<Vec<(usize, usize)>>();

		collisions.into_iter().for_each(|(cell_index, collision_index)| {
			let elm_index = self.cells[collision_index].elm_index;
			let cell = &mut self.cells[cell_index];
			let inter = cell.interactions.clone();

			for interaction in inter {
				if elm_index == interaction.with {
					let chance = self.rng.gen_range(0.0, 1.0);
					if chance < interaction.chance {
						cell.change_element(&self.elements[interaction.into], interaction.into);
					}
				}
			}
		});

		emit.into_iter().for_each(|cell| {
			if self.tree.get(cell.x_r, cell.y_r).is_none() {
				self.cells.push(cell);
			}
		});
	}

	fn set_scroll(&mut self, input: &WinitInputHelper) {
		let scroll = input.scroll_diff();
		self.cursor_radius = (self.cursor_radius as f32 + (scroll * 2.0)).clamp(1.0, 32.0) as usize;
	}

	fn change_selected_element(&mut self, direction: isize) {
		let mut new_elm = self.selected_element;
		while !self.elements[new_elm].drawable || new_elm == self.selected_element {
			let inner_elm = new_elm as isize + direction;
			if inner_elm >= self.elements.len() as isize {
				new_elm = 0;
				continue;
			}
			if inner_elm < 0 {
				new_elm = self.elements.len() - 1;
				continue;
			}
			new_elm = inner_elm as usize;
		}
		self.selected_element = new_elm;
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

		if input.key_pressed(VirtualKeyCode::Down) {
			self.change_selected_element(1);
		}

		if input.key_pressed(VirtualKeyCode::Up) {
			self.change_selected_element(-1);
		}

		input
			.mouse()
			.map(|(mx, my)| {
				self.mouse_x = (mx.clamp(0.0, (SCREEN_WIDTH * SCREEN_SCALE) as f32) / SCREEN_SCALE as f32) as usize;
				self.mouse_y = (my.clamp(0.0, (SCREEN_HEIGHT * SCREEN_SCALE) as f32) / SCREEN_SCALE as f32) as usize;
			})
			.unwrap_or_default();
	}

	fn set_render_time(&mut self) {
		let now = Instant::now();
		self.render_time = now.duration_since(self.last_render).as_secs_f32() * 1000.0;
		self.last_render = now;
	}

	fn create_tree(&mut self) {
		let mut tree = QuadTree::new(0, 0, self.world_width, self.world_height, 500);

		// TODO: Make faster?
		self.cells.iter().enumerate().for_each(|(index, cell)| {
			tree.insert(cell.x_r, cell.y_r, index).unwrap();
		});
		self.tree = tree;
	}

	fn remove_out_of_bounds(&mut self) {
		let height = self.world_height;
		let width = self.world_width;
		self.cells
			.retain(|cell| cell.x_r > 0 && cell.x_r < width && cell.y_r > 0 && cell.y_r < height && cell.alive);
	}

	fn add_element(&mut self) {
		self.is_drawing = false;
		if self.mouse_down_left {
			self.is_drawing = true;
		}

		if self.is_drawing == false {
			return;
		}

		let mx = self.mouse_x;
		let my = self.mouse_y;
		let rad = self.cursor_radius;

		let x_lo = if rad > mx { 0 } else { mx - rad };
		let x_hi = mx + rad;
		let y_lo = if rad > my { 0 } else { my - rad };
		let y_hi = my + rad;

		for px in x_lo..x_hi {
			for py in y_lo..y_hi {
				if !circle_collision(px, py, mx, my, self.cursor_radius, 0.0) {
					continue;
				}

				if px >= self.world_width || py >= self.world_height {
					continue;
				}

				if self.tree.get(px, py).is_some() {
					continue;
				}

				// let index = (py * self.world_width) + px;
				// if self.cells[index].is_some() {
				// 	continue;
				// }

				// self.cells[index] = Some(cell);
				if self.tree.insert(px, py, self.cells.len()).is_ok() {
					let cell = Cell::new(
						&self.elements[self.selected_element],
						self.selected_element,
						px,
						py,
						self.rng.gen(),
					);
					self.cells.push(cell);
				}
			}
		}
	}

	pub fn draw(&mut self, frame: &mut [u8]) {
		// Draw each cell to screen
		self.cells.iter().for_each(|cell| {
			draw::pixel(frame, cell.x_r, cell.y_r, cell.colour);
		});

		// Draw cursor
		if self.mouse_x < self.world_width && self.mouse_y < self.world_height {
			let element = &self.elements[self.selected_element];
			draw::circle(frame, self.mouse_x, self.mouse_y, self.cursor_radius, element.colour);
		}

		// Elements
		let x_off = self.world_width + 2;
		let mut y_counter = 0;
		self.elements.iter().enumerate().for_each(|(index, elm)| {
			if elm.drawable == false {
				return;
			}
			let sep = 12;
			let y_off = (sep * y_counter) + 2;
			y_counter += 1;
			if index == self.selected_element {
				draw::rect(
					frame,
					x_off - 2,
					y_off - 2,
					x_off + GUI_WIDTH as usize - 2,
					y_off + 8,
					[0x66, 0x66, 0x66],
				)
			}
			draw::rect(frame, x_off, y_off, x_off + 6, y_off + 6, elm.colour);
			draw::text(frame, x_off + 8, y_off, &elm.name);
		});

		// UI Border
		draw::rect(
			frame,
			self.world_width,
			0,
			self.world_width + 1,
			self.world_height,
			[0x99, 0x99, 0x99],
		);

		let x_off = self.world_width + 16;
		let y_off = self.world_height - 48;
		draw::rect(frame, x_off - 8, y_off - 8, x_off + 100, y_off + 40, [0x33, 0x33, 0x33]);
		draw::text(
			frame,
			x_off,
			y_off,
			&format!(
				"{} ms ({} fps)",
				self.render_time.ceil(),
				(1000.0 / self.render_time as f32).round()
			),
		);
		draw::text(frame, x_off, y_off + 8, &format!("Cells: {}", self.cells.len()));
		draw::text(frame, x_off, y_off + 16, &format!("Paused: {}", self.is_paused));
		draw::text(frame, x_off, y_off + 24, &format!("Drawing: {}", self.is_drawing));
	}
}
