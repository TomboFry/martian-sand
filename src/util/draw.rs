use rayon::prelude::*;

pub fn clear(frame: &mut [u8]) {
	frame.into_par_iter().for_each(|pixel| {
		*pixel = 0x00;
	});
}

