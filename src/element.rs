use crate::interaction::{Interaction, InteractionJSON};
use crate::RGB;
use rand::prelude::*;

use std::fs::File;
use std::io::BufReader;
// use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ElementJSON {
	pub colour: RGB,
	pub name: String,
	pub density: f32,     // 0 = air, -ve = gas, +ve liquid/solid
	pub viscosity: f32,   // 0 = runny, 1 = completely solid
	pub temperature: f32, // 0 - 100
	pub temperature_delta: Option<f32>,
	pub max_duration: Option<usize>,
	pub interactions: Vec<InteractionJSON>,
	pub drawable: Option<bool>,
	pub emit_element: Option<String>,
	pub emit_chance: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct Element {
	pub colour: RGB,
	pub name: String,
	pub density: f32,     // 0 = air, -ve = gas, +ve liquid/solid
	pub viscosity: f32,   // 0 = runny, 1 = completely solid
	pub temperature: f32, // 0 - 100
	pub temperature_delta: f32,
	pub max_duration: Option<usize>,
	pub interactions: Vec<Interaction>,
	pub drawable: bool,
	pub emit_element: Option<usize>,
	pub emit_chance: f32,
}

impl Element {
	fn find_element(elements: &[Element], name: &str) -> usize {
		let search = elements.iter().enumerate().find(|(_, elm)| elm.name == name);
		if let Some((index, _)) = search {
			return index;
		}
		panic!("Could not find element called {}", name);
	}

	pub fn from_file() -> Vec<Element> {
		let file = File::open("./elements.json");
		if file.is_err() {
			return vec![];
		}
		let file = file.unwrap();

		let rdr = BufReader::new(file);
		let elmjson: Vec<ElementJSON> = serde_json::from_reader(rdr).unwrap();
		let mut elements = elmjson
			.iter()
			.map(|elm| Element {
				colour: elm.colour,
				name: elm.name.clone(),
				density: elm.density,
				viscosity: elm.viscosity,
				temperature: elm.temperature,
				max_duration: elm.max_duration,
				interactions: vec![],
				drawable: elm.drawable.unwrap_or(true),
				emit_element: None,
				emit_chance: elm.emit_chance.unwrap_or(0.0),
				temperature_delta: elm.temperature_delta.unwrap_or(0.0),
			})
			.collect::<Vec<Element>>();

		elmjson.iter().enumerate().for_each(|(index, json)| {
			let interactions = json
				.interactions
				.iter()
				.map(|interaction| Interaction {
					with: Element::find_element(&elements, &interaction.with),
					into: Element::find_element(&elements, &interaction.into),
					chance: interaction.chance,
				})
				.collect::<Vec<Interaction>>();

			elements[index].interactions = interactions;

			if let Some(emit) = &json.emit_element {
				elements[index].emit_element = Some(Element::find_element(&elements, emit));
			}
		});

		elements
	}

	pub fn random(index: usize, rng: &mut ThreadRng) -> Element {
		let max_duration = rng.gen_range(0, 512);
		Element {
			name: format!("Elm {}", index),
			colour: [rng.gen(), rng.gen(), rng.gen()],
			density: rng.gen_range(-0.5, 1.0),
			viscosity: rng.gen_range(0.1, 1.0),
			max_duration: if max_duration > 128 { Some(max_duration) } else { None },
			temperature: (rng.gen_range(-10.0, 100.0) as f32).clamp(0.0, 100.0),
			temperature_delta: 0.0,
			interactions: vec![],
			drawable: true,
			emit_element: None,
			emit_chance: 0.0,
		}
	}
}
