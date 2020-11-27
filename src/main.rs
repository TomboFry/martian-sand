mod cell;
mod element;
mod util;
mod world;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use util::draw;

pub type RGB = [u8; 3];
pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;
pub const GUI_WIDTH: u32 = 128;

fn main() -> Result<(), Error> {
	let event_loop = EventLoop::new();
	let mut input = WinitInputHelper::new();

	// Create Window
	let size = LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT);
	let window = WindowBuilder::new()
		.with_inner_size(size)
		.with_title("Martian Sand")
		.with_resizable(false)
		.build(&event_loop)
		.unwrap();

	let surface_texture = SurfaceTexture::new(SCREEN_WIDTH, SCREEN_HEIGHT, &window);

	// Create world and display for world
	let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?;
	let mut world = world::World::new();

	event_loop.run(move |event, _, control_flow| {
		if let Event::RedrawRequested(_) = event {
			let frame = pixels.get_frame();
			draw::clear(frame);
			world.draw(frame);

			if pixels
				.render()
				.map_err(|e| eprintln!("pixels.render() failed: {}", e))
				.is_err()
			{
				*control_flow = ControlFlow::Exit;
				return;
			}
		}

		if input.update(&event) {
			// Close events
			if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
				*control_flow = ControlFlow::Exit;
				return;
			}

			world.update(&input);
			window.request_redraw();
		}
	});
}
