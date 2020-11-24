use crate::element::Element;

pub struct Cell {
	pub element: Element,
	pub x: usize,
	pub y: usize,
	pub alive: bool,
}

impl Cell {
	pub fn new(element: Element, x: usize, y: usize) -> Cell {
		Cell {
			element,
			x,
			y,
			alive: true,
		}
	}
}
