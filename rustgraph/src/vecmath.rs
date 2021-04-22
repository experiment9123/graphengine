use std::ops::*;
use core::fmt::Debug;
use std::cmp;

type Scalar=f32;
trait VElem : Copy+Clone+Debug+Default+Add+Mul+Div+Sub
{

}
impl<T> VElem for T where T:Add+Mul+Div+Sub+Copy+Clone+Debug+Default{

}

#[derive(Debug,Copy,Clone,Default)]
pub struct Vec2{pub x:Scalar,pub y:Scalar}
#[allow(non_snake_case)]

pub fn Vec2(x:Scalar,y:Scalar)->Vec2{Vec2{x:x,y:y}}
pub trait Dot {
	fn dot(self,b:Self)->Scalar;
	
}
pub trait Splat {
	fn splat(f:Scalar)->Self;
}

pub trait MinMax {
	fn min(self,b:Self)->Self;
	fn max(self,b:Self)->Self;
}

impl Splat for Vec2 {fn splat(f:Scalar)->Self{Vec2{x:f,y:f}}}
impl Splat for Vec3 {fn splat(f:Scalar)->Self{Vec3(f,f,f)}}
impl Splat for Vec4 {fn splat(f:Scalar)->Self{Vec4(f,f,f,f)}}
pub trait VecMath :Splat+Add<Output=Self>+Sub<Output=Self>+Mul<Output=Self>+Mul<Scalar,Output=Self>+Dot+Sized+MinMax+Copy{
	fn lerp(self,b:Self,f:Scalar)->Self;
	fn len(self)->Scalar;
	fn normalize(self)->Self{self*(1.0f32/self.len())}
	fn para_perp(self,axis:Self)->(Self,Self){let d=self.dot(axis); let para=axis*d; (para,self-para)}
}
impl<V:Dot+Copy+Mul<Scalar,Output=V>+Mul<Output=V>+Add<Output=V>+Sub<Output=V>+MinMax+Splat+Sized> VecMath for V {
	fn lerp(self,b:V,f:Scalar)->V {
		self + (b-self)*f
	}
	fn len(self)->Scalar {return self.dot(self).sqrt()}
}

impl MinMax for Vec2{
	fn min(self,b:Self)->Self{
		return Vec2( self.x.min(b.x), self.y.min(b.y) )
	}
	fn max(self,b:Self)->Self{
		return Vec2( self.x.max(b.x), self.y.max(b.y) )
	}
}
impl MinMax for Vec3{
	fn min(self,b:Self)->Self{
		return Vec3( self.x.min(b.x), self.y.min(b.y), self.z.min(b.z) )
	}
	fn max(self,b:Self)->Self{
		return Vec3( self.x.max(b.x), self.y.max(b.y), self.z.max(b.z) )
	}
}
impl MinMax for Vec4{
	fn min(self,b:Self)->Self{
		return Vec4( self.x.min(b.x), self.y.min(b.y), self.z.min(b.z), self.w.min(b.w) )
	}
	fn max(self,b:Self)->Self{
		return Vec4( self.x.max(b.x), self.y.max(b.y), self.z.max(b.z) , self.w.max(b.w))
	}
}



impl Add for Vec2 {
	type Output=Vec2;
	fn add(self,rhs:Vec2)->Vec2 {Vec2(self.x+rhs.x,self.y+rhs.y)}
}
impl Sub for Vec2 {
	type Output=Vec2;
	fn sub(self,rhs:Vec2)->Vec2 {Vec2(self.x-rhs.x,self.y-rhs.y)}
}
impl Mul for Vec2 {
	type Output=Vec2;
	fn mul(self,rhs:Vec2)->Vec2 {Vec2(self.x*rhs.x,self.y*rhs.y)}
}
impl Mul<f32> for Vec2 {
	type Output=Vec2;
	fn mul(self,rhs:Scalar)->Vec2 {Vec2(self.x*rhs,self.y*rhs)}
}
impl Mul<i32> for Vec2 {
	type Output=(i32,i32);
	fn mul(self,rhs:i32)->(i32,i32) {( (self.x*(rhs as f32))as i32,(self.y*(rhs as Scalar))as i32)}
}

impl Dot for Vec2 {
	fn dot(self,b:Vec2)->Scalar {self.x*b.x+self.y*b.y}
}


pub struct Vec3{pub x:Scalar,pub y:Scalar,pub z:Scalar}
#[allow(non_snake_case)]
pub fn Vec3(x:Scalar,y:Scalar,z:Scalar)->Vec3{Vec3{x:x,y:y,z:z}}

impl Add for Vec3 {
	type Output=Vec3;
	fn add(self,rhs:Vec3)->Vec3 {Vec3(self.x+rhs.x,self.y+rhs.y,self.z*rhs.z)}
}
impl Sub for Vec3 {
	type Output=Vec3;
	fn sub(self,rhs:Vec3)->Vec3 {Vec3(self.x-rhs.x,self.y-rhs.y,self.z-rhs.z)}
}
impl Mul for Vec3 {
	type Output=Vec3;
	fn mul(self,rhs:Vec3)->Vec3 {Vec3(self.x*rhs.x,self.y*rhs.y,self.z*rhs.z)}
}
impl Mul<f32> for Vec3 {
	type Output=Vec3;
	fn mul(self,rhs:Scalar)->Vec3 {Vec3(self.x*rhs,self.y*rhs,self.z*rhs)}
}

impl Dot for Vec3 {
	fn dot(self,b:Vec3)->Scalar {self.x*b.x+self.y*b.y+self.z*b.z}
}
#[derive(Copy,Clone,Debug)]
pub struct Vec4{
	pub x:Scalar,pub y:Scalar,pub z:Scalar,pub w:Scalar,
}
#[allow(non_snake_case)]
pub fn Vec4(x:Scalar,y:Scalar,z:Scalar,w:Scalar)->Vec4{Vec4{x:x,y:y,z:z,w:w}}
impl Add for Vec4 {
	type Output=Vec4;
	fn add(self,rhs:Self)->Vec4 {Vec4(self.x+rhs.x,self.y+rhs.y,self.z*rhs.z,self.w*rhs.w)}
}
impl Sub for Vec4 {
	type Output=Vec4;
	fn sub(self,rhs:Vec4)->Vec4 {Vec4(self.x-rhs.x,self.y-rhs.y,self.z-rhs.z,self.w*rhs.w)}
}
impl Mul for Vec4 {
	type Output=Vec4;
	fn mul(self,rhs:Vec4)->Vec4 {Vec4(self.x*rhs.x,self.y*rhs.y,self.z*rhs.z,self.w*rhs.w)}
}
impl Mul<Scalar> for Vec4 {
	type Output=Vec4;
	fn mul(self,rhs:Scalar)->Vec4 {Vec4(self.x*rhs,self.y*rhs,self.z*rhs,self.w*rhs)}
}
impl Dot for Vec4 {
	fn dot(self,b:Vec4)->Scalar {self.x*b.x+self.y*b.y+self.z*b.z+self.w*b.w}
}
impl Vec3 {
	fn cross(self,b:Vec3)->Vec3 {
		let a=self;
		Vec3(a.y*b.z - a.z*b.y, a.z*b.x-a.x*b.z, a.x*b.y-a.y*b.x)
	}
	fn to_vec4(self,w:Scalar)->Vec4{
		Vec4(self.x,self.y,self.z,w)
	}
	pub fn xy(self)->Vec2{Vec2(self.x,self.y)}
	pub fn xz(self)->Vec2{Vec2(self.x,self.z)}
	pub fn yz(self)->Vec2{Vec2(self.y,self.z)}
}

impl Vec4 {
	pub fn cross(self,b:Vec4)->Vec4 {
		let a=self;
		Vec4(a.y*b.z-a.z*b.y, a.z*b.x-a.x*b.z, a.x*b.y-a.y*b.x, 0.0f32)
	}
	pub fn to_vec3(self)->Vec3{
		Vec3(self.x,self.y,self.z)
	}
	pub fn xyz(self)->Vec3{Vec3(self.x,self.y,self.z)}
	pub fn transpose(a:Vec4,b:Vec4,c:Vec4,d:Vec4)->(Vec4,Vec4,Vec4,Vec4){
		(
			Vec4(a.x,b.x,c.x,d.x),
			Vec4(a.y,b.y,c.y,d.y),
			Vec4(a.z,b.z,c.z,d.z),
			Vec4(a.w,b.w,c.w,d.w)
		)
	}
}

struct Extents<V>{min:V,max:V}
impl<V:VecMath> Extents<V>{
	fn new()->Extents<V>{
		Extents{min:V::splat(1000000f32),max:V::splat(-1000000f32)}
	}
	fn size(&self)->V {self.max-self.min}
	fn centre(&self)->V {(self.max+self.min)*0.5f32}
	fn include(&mut self,b:V) {self.min=self.min.min(b); self.max=self.max.max(b)}
}





