#[derive(Clone)]
pub struct Node<T>
where
	T: Clone,
{
	pub x: usize,
	pub y: usize,
	pub data: T,
}

impl<T> Node<T>
where
	T: Clone,
{
	pub fn new(data: T, x: usize, y: usize) -> Node<T> {
		Node { x, y, data }
	}
}
