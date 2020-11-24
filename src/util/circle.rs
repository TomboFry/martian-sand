pub fn circle_collision(px: usize, py: usize, cx: usize, cy: usize, radius: usize, outline: f32) -> bool {
	let d_x = px as f32 - cx as f32;
	let d_y = py as f32 - cy as f32;
	let distance = ((d_x * d_x) + (d_y * d_y)).sqrt();
	if outline <= 0.0 {
		distance < radius as f32
	} else {
		distance < radius as f32 && distance > radius as f32 - outline
	}
}
