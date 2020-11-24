pub struct Element {
	pub name: String,
	pub colour: [u8; 3],

}

impl Element {
	pub fn new(name: &str, colour: [u8; 3]) -> Element {
		Element {
			name: name.to_owned(),
			colour: colour,
		}
	}
}
