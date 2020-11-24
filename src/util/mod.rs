pub mod draw;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn underflow(value: usize) -> usize {
	let max = (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize;
	if value > max {
		return 0;
	}
	value
}
