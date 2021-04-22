#[allow(dead_code)]
#[allow(unused_imports)]

extern crate rustgraph;
extern crate sdl2;
extern crate rand;
use rand::prelude::*;

use rustgraph::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator,BlendMode};
use sdl2::video::{Window, WindowContext};
use rand::Rng;

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
#[derive(Default,Clone,Debug)]
struct Edge{}


#[derive(Default,Clone,Debug)]
struct Cell {pos:Vec2,alive:bool}

impl std::ops::Mul<&Cell> for &Edge {
	type Output=u32;
	fn mul(self,rhs:&Cell)->Self::Output{ if rhs.alive{1u32}else{0u32}}
}


fn init_gol_grid(graph:&mut Graph<Cell,Edge>) {
	let mut rng=rand::thread_rng();
	let spacing:i32=1024/64;
	let gridindex=|x,y|((x&63)+(y&63)*64) as u32;
	for y in 0i32..64i32 {
		for x in 0i32..64i32 {
			graph.add_node(Cell{pos:Vec2((x*spacing+spacing/2) as f32,(y*spacing+spacing/2) as f32),alive: rand::random()});
		}
	}	
	for y in 0i32..64i32 {
		for x in 0i32..64i32 {
			graph.add_edge(Edge{}, gridindex(x,y),gridindex(x+1,y));
			graph.add_edge(Edge{}, gridindex(x,y),gridindex(x-1,y));
			graph.add_edge(Edge{}, gridindex(x,y),gridindex(x,y+1));
			graph.add_edge(Edge{}, gridindex(x,y),gridindex(x,y-1));
			graph.add_edge(Edge{}, gridindex(x,y),gridindex(x+1,y+1));
			graph.add_edge(Edge{}, gridindex(x,y),gridindex(x+1,y-1));
			graph.add_edge(Edge{}, gridindex(x,y),gridindex(x-1,y+1));
			graph.add_edge(Edge{}, gridindex(x,y),gridindex(x-1,y-1));
		}
	}
}
fn pt(p:Vec2)->Point {
	Point::new(p.x as i32,p.y as i32)
}

fn line2d(rc:&mut Canvas<Window>, a:Vec2,b:Vec2){
	rc.draw_line(pt(a),pt(b));
}

struct CompactedJaggedArray<T,Index=usize> {
	first_index:Vec<Index>,
	values:Vec<T>
}

impl<T,Index> CompactedJaggedArray<T,Index> {
	fn new()->Self {return CompactedJaggedArray{first_index:vec![],values:vec![]}}
	fn from_vec(src:&Vec<Vec<T>>)->Self {
		let mut counts:Vec<usize>=vec![0;src.len()];
		for (i,x) in src.iter().enumerate() { counts[i]+=1;}
		return Self::new();		
	}
}


fn main(){
	let mut graph:Graph<Cell,Edge>=  Graph::default();
 	init_gol_grid(&mut graph);

	let va=Vec4(0.1f32,0.5,0.6,1.0);
	let vb=Vec4(0.5,1.0,0.5,1.0);
	let bar=va.dot(vb);
	let vc=va.lerp(vb,0.5);
	
	
	win_stuff(&mut |canvas|{
		graph.update_along_edges(
			|node,&acc|{
				if acc<2 || acc>3{node.alive=false} else
				if acc==3{node.alive=true}
			}
		);

		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();

		let s:i32=2;                                                                                   

		canvas.set_blend_mode(BlendMode::Blend);
		graph.foreach_edge(|e,n0,n1|{
			canvas.set_draw_color(Color::RGBA(128, 128,128,16));
			line2d(canvas,n0.pos,n1.pos);

		});

		graph.foreach_node(|n|{
			canvas.set_draw_color(if n.alive {Color::RGB(255,255,255)}else{Color::RGB(64,64,64)});
			canvas.fill_rect(Rect::new(n.pos.x as i32-s,n.pos.y as i32 -s,s as u32*2,s as u32*2));
		});

		canvas.present();	
	});

}

