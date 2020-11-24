use crate::util::font::*;
use crate::util::underflow;
use crate::SCREEN_WIDTH;
use rayon::prelude::*;

/// Set every pixel on the screen to black. This task is parallelised
pub fn clear(frame: &mut [u8]) {
	frame.into_par_iter().for_each(|pixel| {
		*pixel = 0x00;
	});
}

/// Determine the index for any given point on the screen.
/// This factors in the fact that each pixel uses 4 bytes for colour (rgba).
fn get_index(x: usize, y: usize) -> usize {
	(x + (y * SCREEN_WIDTH as usize)) * 4
}

/// Draw a single pixel, with a given colour, to the screen at a given point
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

fn circle_main(frame: &mut [u8], cx: usize, cy: usize, radius: usize, colour: [u8; 3], outline: f32) {
	let x_lo = underflow(cx - radius);
	let x_hi = underflow(cx + radius);
	let y_lo = underflow(cy - radius);
	let y_hi = underflow(cy + radius);

	for px in x_lo..x_hi {
		for py in y_lo..y_hi {
			let d_x = px as f32 - cx as f32;
			let d_y = py as f32 - cy as f32;
			let distance = ((d_x * d_x) + (d_y * d_y)).sqrt();
			let should_draw = if outline <= 0.0 {
				distance < radius as f32
			} else {
				distance < radius as f32 && distance > radius as f32 - outline
			};
			if should_draw {
				pixel(frame, px, py, colour);
			}
		}
	}
}

/// Draw a filled circle to the screen
pub fn circle(frame: &mut [u8], cx: usize, cy: usize, radius: usize, colour: [u8; 3]) {
	circle_main(frame, cx, cy, radius, colour, 0.0);
}

/// Draw an outlined circle with a given thickness
pub fn circle_outline(frame: &mut [u8], cx: usize, cy: usize, radius: usize, colour: [u8; 3], thickness: f32) {
	circle_main(frame, cx, cy, radius, colour, thickness);
}

/// Draw a single letter to the screen based on the blit32 font
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

/// Draw a string of text to the screen.
/// This will ignore any characters outside of the range of valid characters.
pub fn text(frame: &mut [u8], x: usize, y: usize, text: &str) {
	text.chars()
		.filter_map(|letter| {
			let code = letter as usize;
			if code < 32 {
				return None;
			}
			let index = code - 32;
			if index > 95 {
				return None;
			}
			Some(FONT[index])
		})
		.enumerate()
		.for_each(|(tx, index)| {
			letter(frame, (tx * 8) + x, y, index, [0xff, 0xff, 0xff]);
		});
}
