use crate::RGB;

#[derive(Clone)]
pub struct Element {
	pub name: String,
	pub colour: RGB,
}

impl Element {
	pub fn new(name: &str, colour: RGB) -> Element {
		Element {
			name: name.to_owned(),
			colour,
		}
	}
}
