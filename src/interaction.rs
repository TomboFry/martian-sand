use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct InteractionJSON {
	pub with: String, // Name of element
	pub into: String, // Name of element
	pub chance: f32,  // 0 - 1
}

#[derive(Clone, Copy, Debug)]
pub struct Interaction {
	pub with: usize, // Element index
	pub into: usize, // Element index
	pub chance: f32, // 0 - 1
}
