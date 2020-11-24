use crate::util::font::*;
use crate::util::underflow;
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

pub fn circle(frame: &mut [u8], cx: usize, cy: usize, radius: usize, colour: [u8; 3]) {
	let x_lo = underflow(cx - radius);
	let x_hi = underflow(cx + radius);
	let y_lo = underflow(cy - radius);
	let y_hi = underflow(cy + radius);

	for px in x_lo..x_hi {
		for py in y_lo..y_hi {
			let d_x = px as f32 - cx as f32;
			let d_y = py as f32 - cy as f32;
			let distance = ((d_x * d_x) + (d_y * d_y)).sqrt();
			if distance < radius as f32 {
				pixel(frame, px, py, colour);
			}
		}
	}
}
fn letter(frame: &mut [u8], x: usize, y: usize, letter: u32, colour: [u8; 3]) {
	for line_offset in 0..FONT_HEIGHT {
		for letter_offset in 0..FONT_WIDTH {
			let shift = (line_offset * FONT_WIDTH) + letter_offset;
			// Shift the bits and mask everything but the smallest bit
			// (essentially a boolean at this point)
			let chr = (letter >> shift) & 0b00000001;
			if chr == 1 {
				pixel(frame, x + letter_offset as usize, y + line_offset as usize, colour);
			}
		}
	}
}

pub fn text(frame: &mut [u8], x: usize, y: usize, text: &str) {
	text.chars()
		.filter_map(|letter| {
			if !letter.is_ascii() {
				return None;
			}
			let index = (letter as usize) - 32;
			Some(FONT[index])
		})
		.enumerate()
		.for_each(|(tx, index)| {
			letter(frame, (tx * 8) + x, y, index, [0xff, 0xff, 0xff]);
		});
}
