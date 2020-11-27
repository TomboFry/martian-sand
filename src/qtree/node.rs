use crate::qtree::space::Point;

#[derive(Clone, Debug)]
pub struct Node<T>
where
	T: Clone,
{
	pub pos: Point,
	pub data: T,
}

impl<T> Node<T>
where
	T: Clone,
{
	pub fn new(data: T, x: usize, y: usize) -> Node<T> {
		Node {
			pos: Point { x, y },
			data,
		}
	}
}
