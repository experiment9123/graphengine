extern crate rustgraph;
mod window;
use rustgraph::*;
struct RenderCtx{}

#[derive(Debug,Clone,Default)]
struct Ctrl {
	lstick:Vec2,
	rstick:Vec2,
	buttons:u32
}

#[derive(Default)]
struct ActorBase {
	pos:Vec3,vel:Vec3,radius:f32,lifetime:i32
}

struct UpdateCtx {
	dt:f32,
	controls:Ctrl
}
struct Spawner {} 

// if it doesn't have a render,
// then it is infact jsut..
type OptBox<T>=Option<Box<T>>;

trait Agent {
	fn control(&mut self, obj:&ActorBase)->Ctrl;
}

// caution we're up to 32bytes of bloat with OptBox for an agent :/ 2*2*8
// 'actorbase' will be matrix, vel,pos, renderable ptr, lifetime.. 64+32+8.. whole thing should come in ~128-256 bytes.
struct Actor{base:ActorBase,ex:OptBox<dyn ActorEx>,agent:OptBox<dyn Agent>}
type AParams<'a,'b,'c> = (&'a mut ActorBase,&'b Ctrl,f32,&'c mut Spawner);


trait ActorEx {
	fn update(&mut self,_:AParams);
	fn collide(&mut self,sc:&ActorBase,other:&ActorBase,spawner:&mut Spawner){}
}

struct Null {}
impl ActorEx for Null {
	fn update(&mut self,_:AParams){}
}

struct World {
	particles:Vec<Actor>,	//no collision
	projectiles:Vec<Actor>,	//collide vs entities
	entities:Vec<Actor>,	//collide v eachother.
}

impl World {
	fn update(&mut self,uc:&UpdateCtx){
		let update_sub=|list:&mut Vec<Actor>|{
			let mut s=Spawner{};
			for x in list.iter_mut() {
				if let Some(ref mut ex)=&mut x.ex{
					let ctrl= if let Some(ref mut a)=&mut x.agent{a.control(&x.base)} else{uc.controls.clone()};
					ex.update((&mut x.base,&ctrl,uc.dt,&mut s));
				}
			}
		};
		update_sub(&mut self.particles);
  		update_sub(&mut self.projectiles);
		update_sub(&mut self.entities);
	}
}
fn main(){
	window::foo();

	println!("done\n");	
}


