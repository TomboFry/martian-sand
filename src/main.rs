mod element;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;

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
	event_loop.run(move |event, _, control_flow| {
		if let Event::RedrawRequested(_) = event {
			// TODO: DRAW STUFF HERE

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

			// TODO: Update world here
			window.request_redraw();
		}
	});
}
