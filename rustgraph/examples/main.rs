extern crate rustgraph;
mod window;
mod world;

use rustgraph::*;
fn main(){
	let world=world::World::default();

	let v=Vec4(0.5f32,0.5f32,0.5f32,1.0f32);
	let v2=Vec4(0.2f32,0.4f32,0.2f32,0.0f32);
	let p=v.pack();
	let v2=Vec4::unpack(p);
	let v3=v2.normalize();
	let v4=v3.cross(v2);
	println!("{:?},{:x},{:?},{:?}.{:?},{:?}",v,p,v2,v3,v3.dot(v3),v4);
	println!("main/done\n");	
}


