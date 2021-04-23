#[allow(dead_code)]
#[allow(unused_imports)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator,BlendMode};
use sdl2::video::{Window, WindowContext};

pub fn win_stuff(renderf:&mut dyn FnMut(&mut Canvas<Window>))->Result<(),String> where
{
	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	// the window is the representation of a window in your operating system,
	// however you can only manipulate properties of that window, like its size, whether it's
	// fullscreen, ... but you cannot change its content without using a Canvas or using the
	// `surface()` method.
	let window = video_subsystem
	.window(
	    "rust-sdl2 window",
	    1024,
	    1024,
	)
	.position_centered()
	.build()
	.map_err(|e| e.to_string())?;

	// the canvas allows us to both manipulate the property of the window and to change its content
	// via hardware or software rendering. See CanvasBuilder for more info.
	let mut canvas = window
	.into_canvas()
	.target_texture()
	.present_vsync()
	.build()
	.map_err(|e| e.to_string())?;

	println!("Using SDL_Renderer \"{}\"", canvas.info().name);
	let mut event_pump = sdl_context.event_pump()?;

	'mainloop: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
				    keycode: Some(Keycode::Escape),
				    ..
				} => break 'mainloop,
				Event::KeyDown {keycode: Some(Keycode::Space),repeat: false,..} => {

				}
				Event::MouseButtonDown{x,y,mouse_btn: MouseButton::Left,..} => {
				}
				_ => {}
			}
		};

		// clears the canvas with the color we set in `set_draw_color`.
		renderf(&mut canvas);
			}
	Ok(())
}
