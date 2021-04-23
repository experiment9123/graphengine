use std::ops::*;
use core::fmt::Debug;
use std::cmp;

type Scalar=f32;
pub trait VElem : Copy+Clone+Debug+Default+Neg<Output=Self>+Add<Output=Self>+Mul<f32,Output=f32>+Into<f32>+From<f32>+Mul<Output=Self>+Div<Output=Self>+Sub<Output=Self> + PartialOrd
{

}
impl<T> VElem for T where T:Into<f32>+From<f32>+PartialOrd+Neg<Output=T>+Add<Output=T>+Mul<Output=T>+Mul<f32,Output=f32>+Div<Output=T>+Sub<Output=T>+Copy+Clone+Debug+Default{

}

pub trait OneZero :Copy{fn one()->Self;	fn zero()->Self;}
impl OneZero for f32{	fn one()->Self{1.0f32}	fn zero()->Self{0.0f32}}
impl OneZero for f64{	fn one()->Self{1.0f64}	fn zero()->Self{0.0f64}}

#[derive(Debug,Copy,Clone,Default)]
pub struct Vec2<T=f32>{pub x:T,pub y:T}
#[derive(Copy,Clone,Debug,Default)]
pub struct Vec3<T=f32>{pub x:T,pub y:T,pub z:T}

#[allow(non_snake_case)]
pub fn Vec3<T>(x:T,y:T,z:T)->Vec3<T>{Vec3{x:x,y:y,z:z}}


#[allow(non_snake_case)]

pub fn Vec2<T>(x:T,y:T)->Vec2<T>{Vec2{x:x,y:y}}
pub trait Dot :Sized{
	type Output : Into<f32>;
	fn dot(self,b:Self)->Self::Output;
	fn dot_f32(self,b:Self)->f32 {self.dot(b).into()}
	
}

fn fmin<T:Copy+PartialOrd>(a:T,b:T)->T{if a<b{a} else {b}}
fn fmax<T:Copy+PartialOrd>(a:T,b:T)->T{if a>b{a} else {b}}

pub trait MinMax {
	fn min(self,b:Self)->Self;
	fn max(self,b:Self)->Self;
}
impl<T:Copy+PartialOrd> MinMax for Vec2<T>{
	fn min(self,b:Self)->Self{
		return Vec2( fmin(self.x,b.x), fmin(self.y,b.y) )
	}
	fn max(self,b:Self)->Self{
		return Vec2( fmax(self.x,b.x), fmax(self.y,b.y) )
	}
}
impl<T:VElem> Add for Vec2<T> {
	type Output=Vec2<T>;
	fn add(self,rhs:Vec2<T>)->Self::Output {Vec2(self.x+rhs.x,self.y+rhs.y)}
}
impl<T:VElem> Sub for Vec2<T> {
	type Output=Vec2<T>;
	fn sub(self,rhs:Vec2<T>)->Self::Output {Vec2(self.x-rhs.x,self.y-rhs.y)}
}
impl<T:VElem> Mul for Vec2<T> {
	type Output=Vec2<T>;
	fn mul(self,rhs:Vec2<T>)->Self::Output {Vec2(self.x*rhs.x,self.y*rhs.y)}
}
impl<T:VElem> Mul<T> for Vec2<T> {
	type Output=Vec2<T>;
	fn mul(self,rhs:T)->Self::Output {Vec2(self.x*rhs,self.y*rhs)}
}
impl Mul<i32> for Vec2<f32> {
	type Output=(i32,i32);
	fn mul(self,rhs:i32)->Self::Output {( (self.x*(rhs as f32))as i32,(self.y*(rhs as Scalar))as i32)}
}

impl<T:VElem> Dot for Vec2<T> {
	type Output=T;
	fn dot(self,b:Self)->T {self.x*b.x+self.y*b.y}
}
fn clamp<T:PartialOrd+Copy>(val:T,lo:T,hi:T)->T{fmin(hi,fmax(lo,val))}
impl Vec4<f32> {
	pub fn unpack(v:u32)->Self {
		let s=1.0f32/255.0f32;
		let x=v&255;
		let y=(v>>8)&255;
		let z=(v>>16)&255;
		let w=(v>>24)&255;
		Vec4(x as f32 * s,y as f32* s,z as f32* s,w  as f32* s)

	}
	pub fn pack(self)->u32 {
		let s=255.0f32;
		let x=clamp((self.x*s) as u32,0,255);
		let y=clamp((self.y*s) as u32,0,255);
		let z=clamp((self.z*s) as u32,0,255);
		let w=clamp((self.w*s) as u32,0,255);
		x|(y<<8)|(z<<16)|(w<<24)
	}
}

pub trait Splat<T> { fn splat(f:T)->Self;}
impl<T:Copy> Splat<T> for Vec2<T>{ fn splat(f:T)->Self{return Vec2(f,f);} }
impl<T:Copy> Splat<T> for Vec3<T>{ fn splat(f:T)->Self{return Vec3(f,f,f);} }
impl<T:Copy> Splat<T> for Vec4<T>{ fn splat(f:T)->Self{return Vec4(f,f,f,f);} }


pub trait Scale { fn scale(self,f:f32)->Self;}
impl<T:Into<f32>+From<f32> > Scale for Vec3<T>{
	fn scale(self,f:f32)->Self{
			let x:f32=self.x.into();
			let y:f32=self.y.into();
			let z:f32=self.z.into();
			Vec3((x*f).into(),(y*f).into(),(z*f).into())
	}
}
impl<T:Into<f32>+From<f32> > Scale for Vec4<T>{
	fn scale(self,f:f32)->Self{
			let x:f32=self.x.into();
			let y:f32=self.y.into();
			let z:f32=self.z.into();
			let w:f32=self.w.into();
			Vec4((x*f).into(),(y*f).into(),(z*f).into(),(w*f).into())
	}
}

pub trait Cross :Copy{
	fn cross(self,other:Self)->Self;
}
impl<T:VElem+OneZero> Cross for Vec3<T>{
	fn cross(self,other:Self)->Self{
		Vec3(
			self.y*other.z-self.z*other.y,
			self.z*other.x-self.x*other.z,
			self.x*other.y-self.y*other.x
		)
	}
}
impl<T:VElem+OneZero> Cross for Vec4<T>{
	fn cross(self,other:Self)->Self{
		Vec4(
			self.y*other.z-self.z*other.y,
			self.z*other.x-self.x*other.z,
			self.x*other.y-self.y*other.x,
			T::zero()
		)
	}
}


pub trait VecMath : Copy+Add<Output=Self>+Sub<Output=Self>+Mul<Output=Self>+Dot+Scale+MinMax
{
	fn len(self)->f32{self.dot_f32(self).sqrt()	}
	fn normalize(self)->Self{self.scale(1.0f32/self.len())}
	fn lerp(self,b:Self,f:f32)->Self{(b-self).scale(f)+self}
	fn para(self,axis:Self)->Self{self.scale(self.dot_f32(axis))}
	fn para_perp(self,axis:Self)->(Self,Self){let para=self.para(axis); (para,self-para)}
	fn perp(self,axis:Self)->Self{self-self.para(axis)}
	fn sqr(self)->f32{self.dot_f32(self)}

}

impl<V> VecMath for V where
V:Copy+Add<Output=Self>+Sub<Output=Self>+Mul<Output=Self>+Dot+Scale+MinMax
{

}

impl<T:OneZero>  Vec4<T>{
	fn xyz0(self)->Vec4<T>{Vec4(self.x,self.y,self.z,T::zero())}
	fn xyz1(self)->Vec4<T>{Vec4(self.x,self.y,self.z,T::one())}
	fn xyz(self)->Vec3<T>{Vec3(self.x,self.y,self.z)}
}

impl<T:OneZero>  Vec3<T>{
	fn xyz0(self)->Vec4<T>{Vec4(self.x,self.y,self.z,T::zero())}
	fn xyz1(self)->Vec4<T>{Vec4(self.x,self.y,self.z,T::one())}
}


impl<T:VElem> Add for Vec3<T> {
	type Output=Self;
	fn add(self,rhs:Self)->Self::Output {Vec3(self.x+rhs.x,self.y+rhs.y,self.z*rhs.z)}
}
impl<T:VElem> Sub for Vec3<T> {
	type Output=Self;
	fn sub(self,rhs:Self)->Self::Output {Vec3(self.x-rhs.x,self.y-rhs.y,self.z-rhs.z)}
}
impl<T:VElem> Mul for Vec3<T> {
	type Output=Self;
	fn mul(self,rhs:Self)->Self::Output {Vec3(self.x*rhs.x,self.y*rhs.y,self.z*rhs.z)}
}
impl<T:VElem> Mul<T> for Vec3<T> {
	type Output=Self;
	fn mul(self,rhs:T)->Self::Output {Vec3(self.x*rhs,self.y*rhs,self.z*rhs)}
}

impl<T:VElem> Dot for Vec3<T> {
	type Output=T;
	fn dot(self,b:Self)->Self::Output {self.x*b.x+self.y*b.y+self.z*b.z}
}

#[derive(Copy,Clone,Debug,Default)]
pub struct Vec4<T=f32>{
	pub x:T,pub y:T,pub z:T,pub w:T,
}
#[allow(non_snake_case)]
pub fn Vec4<T>(x:T,y:T,z:T,w:T)->Vec4<T>{Vec4{x:x,y:y,z:z,w:w}}

impl<T:VElem> Add for Vec4<T> {
	type Output=Self;
	fn add(self,rhs:Self)->Self::Output {Vec4(self.x+rhs.x,self.y+rhs.y,self.z*rhs.z,self.w*rhs.w)}
}
impl<T:VElem> Sub for Vec4<T> {
	type Output=Self;
	fn sub(self,rhs:Self)->Self::Output {Vec4(self.x-rhs.x,self.y-rhs.y,self.z-rhs.z,self.w-rhs.w)}
}
impl<T:VElem> Mul for Vec4<T> {
	type Output=Self;
	fn mul(self,rhs:Self)->Self::Output {Vec4(self.x*rhs.x,self.y*rhs.y,self.z*rhs.z,self.w*rhs.w)}
}
impl<T:VElem> Mul<T> for Vec4<T> {
	type Output=Self;
	fn mul(self,rhs:T)->Self::Output {Vec4(self.x*rhs,self.y*rhs,self.z*rhs,self.w*rhs)}
}

impl<T:VElem> Dot for Vec4<T> {
	type Output=T;
	fn dot(self,b:Self)->Self::Output {self.x*b.x+self.y*b.y+self.z*b.z+self.w*b.w}
}

impl<T:VElem> MinMax for Vec3<T>{
	fn min(self,b:Self)->Self{
		return Vec3( fmin(self.x,b.x), fmin(self.y,b.y), fmin(self.z,b.z) )
	}
	fn max(self,b:Self)->Self{
		return Vec3( fmax(self.x,b.x), fmax(self.y,b.y), fmax(self.z,b.z) )
	}
}

impl MinMax for Vec4{
	fn min(self,b:Self)->Self{
		return Vec4( fmin(self.x,b.x), fmin(self.y,b.y), fmin(self.z,b.z), fmin(self.w,b.w)  )
	}
	fn max(self,b:Self)->Self{
		return Vec4( fmax(self.x,b.x), fmax(self.y,b.y), fmax(self.z,b.z), fmax(self.w,b.w)  )
	}
}


struct Vec1<T>{x:T}
fn Vec1<T>(x:T)->Vec1<T>{Vec1{x:x}}
impl<T:Copy+PartialOrd> MinMax for Vec1<T>{
	fn min(self,b:Self)->Self{return Vec1(fmin(self.x,b.x));}
	fn max(self,b:Self)->Self{return Vec1(fmax(self.x,b.x));}
}


struct Extents<V>{min:V,max:V}
impl<V:Copy+MinMax+Add<Output=V>+Sub<Output=V>+Scale+Splat<f32>> Extents<V>{
	fn new()->Extents<V>{
		Extents{min:V::splat(1000000f32),max:V::splat(-100000f32)}
	}
	fn size(&self)->V {self.max-self.min}
	fn centre(&self)->V {(self.max+self.min).scale(0.5f32)}
	fn include(&mut self,b:V) {self.min=self.min.min(b); self.max=self.max.max(b)}
}

#[derive(Debug,Copy,Clone,Default)]
struct Matrix4<V=Vec4<f32>>{ax:V,ay:V,az:V,aw:V}
fn Matrix4<V>(x:V,y:V,z:V,w:V)->Matrix4<V>{Matrix4{ax:x,ay:y,az:z,aw:w}}



impl<T:VElem> Mul<Vec4<T>> for Matrix4<Vec4<T>> {
	type Output=Vec4<T>;
	fn mul(self,rhs:Vec4<T>)->Self::Output{
		self.ax*rhs.x+self.ay*rhs.y+self.az*rhs.z+self.aw*rhs.w
	}
}
impl<T:VElem+OneZero> Mul<Vec3<T>> for Matrix4<Vec4<T>> {
	type Output=Vec3<T>;
	fn mul(self,rhs:Vec3<T>)->Self::Output{
		(self.ax*rhs.x+self.ay*rhs.y+self.az*rhs.z).xyz()
	}
}

impl<T:VElem> Mul for Matrix4<Vec4<T>> {
	type Output=Self;
	fn mul(self,rhs:Self)->Self{
		Matrix4(self*rhs.ax,self*rhs.ay,self*rhs.az,self*rhs.aw)
	}
}

impl<T:VElem+OneZero> Matrix4<Vec4<T>>{
	fn transpose(self)->Self{
		Matrix4(
			Vec4(self.ax.x,self.ay.x,self.az.x,self.aw.x),
			Vec4(self.ax.y,self.ay.y,self.az.y,self.aw.y),
			Vec4(self.ax.z,self.ay.z,self.az.z,self.aw.z),
			Vec4(self.ax.w,self.ay.w,self.az.w,self.aw.w)
		)
	}
	fn identity()->Self{
		let i=T::one();
		let o=T::zero();
		Matrix4(
			Vec4(i,o,o,o),
			Vec4(o,i,o,o),
			Vec4(o,o,i,o),
			Vec4(o,o,o,i),
		)
	}
	fn scale_vec4(v:Vec4<T>)->Self{
		let o=T::zero();
		Matrix4(
			Vec4(v.x,o,o,o),
			Vec4(o,v.y,o,o),
			Vec4(o,o,v.z,o),
			Vec4(o,o,o,v.w),
		)		
	}
	fn rotx(a:f32)->Self{
		let s=a.sin().into();
		let c=a.cos().into();
		let o=T::zero();
		let i=T::one();
		Matrix4(
			Vec4(i,o,o,o),
			Vec4(o,c,s,o),
			Vec4(o,-s,c,o),
			Vec4(o,o,o,i),
		)		
	}
	fn roty(a:f32)->Self{
		let s=a.sin().into();	let c=a.cos().into();
		let o=T::zero();		let i=T::one();
		Matrix4(
			Vec4(c,o,s,o),
			Vec4(o,i,o,o),
			Vec4(-s,o,c,o),
			Vec4(o,o,o,i),
		)		
	}
	fn rotz(a:f32)->Self{
		let s=a.sin().into();	let c=a.cos().into();
		let o=T::zero();		let i=T::one();
		Matrix4(
			Vec4(c,s,o,o),
			Vec4(-s,c,o,o),
			Vec4(o,o,o,o),
			Vec4(o,o,o,i),
		)		
	}

	fn look_at(pos:Vec3<T>,tgt:Vec3<T>,up:Vec3<T>)->Self{
		Self::look_along(pos,tgt-pos,up)
	}
	fn look_along(pos:Vec3<T>,fwd1:Vec3<T>,up:Vec3<T>)->Self{
		let i=T::one();
		let o=T::zero();
		let fwd=fwd1.normalize();
		let horiz=fwd.cross(up).normalize();
		let ay=fwd.cross(horiz).normalize();
		Matrix4(horiz.xyz0(),ay.xyz0(),fwd.xyz0(),pos.xyz1())
	}
	fn mul_point(self,rhs:Vec3<T>)->Vec3<T>{
		(self.ax*rhs.x+self.ay*rhs.y+self.az*rhs.z+self.aw).xyz()
	}
	fn inv_mul_point_rt(self,rhs:Vec3<T>)->Vec3<T>{
		let ofs=(rhs-self.aw.xyz()).xyz0();

		Vec3(ofs.dot(self.ax),ofs.dot(self.ay),ofs.dot(self.az)	)
	}
	fn orthonormalize_zyx(self)->Self{
		Self::look_along(self.aw.xyz(),self.az.xyz(),self.ay.xyz())
	}

}





