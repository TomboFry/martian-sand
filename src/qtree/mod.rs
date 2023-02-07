#[derive(Clone)]
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

pub struct Point<T> {
	pub x: usize,
	pub y: usize,
	pub data: T,
}

pub enum Node<T> {
	Leaf(Vec<Point<T>>),
	Subtree(Vec<QuadTree<T>>),
}

pub struct QuadTree<T> {
	pub rect: Rect,
	pub capacity: usize,
	pub node: Node<T>,
}

impl<T> QuadTree<T>
where
	T: Copy,
{
	pub fn new(x1: usize, y1: usize, x2: usize, y2: usize, capacity: usize) -> QuadTree<T> {
		let rect = Rect { x1, y1, x2, y2 };

		QuadTree {
			rect,
			capacity,
			node: Node::Leaf(Vec::with_capacity(capacity)),
		}
	}

	fn subdivide(&mut self) {
		if let Node::Subtree(_) = self.node {
			panic!("Trying to subdivide a child that's already been divided!");
		}

		let x1 = self.rect.x1;
		let y1 = self.rect.y1;
		let x2 = self.rect.x2;
		let y2 = self.rect.y2;
		let capacity = self.capacity;

		let mut children: Vec<QuadTree<T>> = Vec::with_capacity(4);

		let nw = QuadTree::new(x1, y1, (x1 + x2) / 2, (y1 + y2) / 2, capacity);
		let ne = QuadTree::new((x1 + x2) / 2, y1, x2, (y1 + y2) / 2, capacity);
		let sw = QuadTree::new(x1, (y1 + y2) / 2, (x1 + x2) / 2, y2, capacity);
		let se = QuadTree::new((x1 + x2) / 2, (y1 + y2) / 2, x2, y2, capacity);

		children.push(nw);
		children.push(ne);
		children.push(sw);
		children.push(se);

		let mut points = Node::Subtree(children);
		std::mem::swap(&mut points, &mut self.node);

		if let Node::Leaf(items) = points {
			for Point { x, y, data } in items.into_iter() {
				self.insert(x, y, data).unwrap();
			}
		}
	}

	pub fn insert(&mut self, x: usize, y: usize, data: T) -> Result<(), ()> {
		if self.rect.contains(x, y) == false {
			return Err(());
		}

		match &mut self.node {
			Node::Leaf(points) => {
				if points.len() >= self.capacity {
					self.subdivide();
					return self.insert(x, y, data);
				}

				points.push(Point { x, y, data });
				return Ok(());
			}
			Node::Subtree(quads) => {
				for quad in quads.iter_mut() {
					if let Ok(()) = quad.insert(x, y, data) {
						return Ok(());
					}
				}

				unreachable!("All insertions failed");
			}
		};
	}

	pub fn get<'a>(&'a self, x: usize, y: usize) -> Option<T> {
		if !self.rect.contains(x, y) {
			return None;
		}

		match &self.node {
			Node::Subtree(quads) => {
				for quad in quads.iter() {
					if let Some(v) = quad.get(x, y) {
						return Some(v);
					}
				}
			}
			Node::Leaf(points) => {
				for point in points.iter() {
					if point.x == x && point.y == y {
						return Some(point.data);
					}
				}
			}
		};

		None
	}

	pub fn remove(&mut self, x: usize, y: usize) -> Result<(), ()> {
		if !self.rect.contains(x, y) {
			return Err(());
		}

		match &mut self.node {
			Node::Subtree(quads) => {
				for quad in quads {
					if quad.remove(x, y).is_ok() {
						return Ok(());
					}
				}
			}
			Node::Leaf(points) => {
				points.retain(|point| point.x != x && point.y != y);
				return Ok(());
			}
		}

		Err(())
	}
}
