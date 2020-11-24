use crate::SCREEN_WIDTH;
use rayon::prelude::*;

pub fn clear(frame: &mut [u8]) {
	frame.into_par_iter().for_each(|pixel| {
		*pixel = 0x00;
	});
}

fn get_index(x: usize, y: usize) -> usize {
	(x + (y * SCREEN_WIDTH as usize)) * 4
}

pub fn pixel(frame: &mut [u8], x: usize, y: usize, colour: [u8; 3]) {
	let idx = get_index(x, y);

	if idx >= frame.len() {
		return;
	}

	frame[idx] = colour[0];
	frame[idx + 1] = colour[1];
	frame[idx + 2] = colour[2];
	frame[idx + 3] = 0xff;
}

