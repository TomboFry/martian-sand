use crate::qtree::node::Node;
use crate::qtree::space::{Point, Rect};

pub struct QuadTree<T>
where
	T: Clone,
{
	pub rect: Rect,
	pub capacity: usize,
	pub data: Vec<Node<T>>,
	pub quads: Vec<QuadTree<T>>,
}

impl<T> QuadTree<T>
where
	T: Clone,
{
	pub fn new(x1: usize, y1: usize, x2: usize, y2: usize, capacity: usize) -> QuadTree<T> {
		let rect = Rect { x1, y1, x2, y2 };

		QuadTree {
			rect,
			capacity,
			data: vec![],
			quads: vec![],
		}
	}

	fn subdivide(&mut self) {
		let x1 = self.rect.x1;
		let y1 = self.rect.y1;
		let x2 = self.rect.x2;
		let y2 = self.rect.y2;
		let capacity = self.capacity;

		let nw = QuadTree::new(x1, y1, (x1 + x2) / 2, (y1 + y2) / 2, capacity);
		let ne = QuadTree::new((x1 + x2) / 2, y1, x2, (y1 + y2) / 2, capacity);
		let sw = QuadTree::new(x1, (y1 + y2) / 2, (x1 + x2) / 2, y2, capacity);
		let se = QuadTree::new((x1 + x2) / 2, (y1 + y2) / 2, x2, y2, capacity);

		self.quads.push(nw);
		self.quads.push(ne);
		self.quads.push(sw);
		self.quads.push(se);
	}

	pub fn insert(&mut self, node: Node<T>) {
		if self.rect.contains(&node.pos) == false {
			return;
		}

		if self.data.len() < self.capacity {
			self.data.push(node);
			return;
		}

		if self.quads.len() == 0 {
			self.subdivide();
		}

		for quad in &mut self.quads {
			quad.insert(node.clone());
		}
	}

	pub fn find(&self, x: usize, y: usize) -> Option<&Node<T>> {
		if !self.rect.contains(&Point { x, y }) {
			return None;
		}

		let data = self.data.iter().find(|cell| cell.pos.x == x && cell.pos.y == y);
		if data.is_some() {
			return data;
		}

		// Recurse over each quad until it is found
		self.quads.iter().fold(None, |opt, quad| quad.find(x, y).or(opt))
	}
}
