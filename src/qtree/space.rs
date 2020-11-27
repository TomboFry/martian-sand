#[derive(Clone)]
pub struct Point {
	pub x: usize,
	pub y: usize,
}

pub struct Rect {
	pub x1: usize,
	pub y1: usize,
	pub x2: usize,
	pub y2: usize,
}

impl Rect {
	pub fn contains(&self, point: &Point) -> bool {
		point.x >= self.x1 && point.x < self.x2 && point.y >= self.y1 && point.y < self.y2
	}
}
