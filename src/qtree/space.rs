pub struct Rect {
	pub x1: usize,
	pub y1: usize,
	pub x2: usize,
	pub y2: usize,
}

impl Rect {
	pub fn contains(&self, x: usize, y: usize) -> bool {
		x >= self.x1 && x < self.x2 && y >= self.y1 && y < self.y2
	}
}
