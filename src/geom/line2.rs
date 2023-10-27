use std::ops::{Rem, Div, Sub, Mul, Neg, Add};

use crate::math::{numth::gcd, structs::{Zero, One}};

use super::vec2::Vec2;

pub enum LineErr{
	Same,
	Parallel
}

//NOTE: 직선의 방정식 (ax+by+c=0)의 2by2연립solver관점에서 작성됨.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line2<T>{
	pub a:T,pub b:T,pub c:T
}
impl<T> Line2<T>
where T:Copy+PartialEq+Zero<T>+One<T>
+Add<Output=T>+Mul<Output=T>+Neg<Output=T>+Sub<Output=T>
+Div<Output=T>+Rem<Output=T>{
	pub fn from_2pt(v0:Vec2::<T>, v1:Vec2::<T>)->Self{
		Self{a:v0.y-v1.y, b:v1.x-v0.x, c:-v0.y*(v1.x-v0.x)+v0.x*(v1.y-v0.y)}
	}
	pub fn from_tan_yic(tan:T, yic:T)->Self{
		Self::from_2pt(Vec2{x:T::zero(),y:yic}, Vec2{x:T::one(),y:yic+tan})
	}
	pub fn normalized(&self)->Self{
		let g = gcd(self.a, self.b);
		Self{a:self.a/g,b:self.b/g,c:self.c/g}
	}
	pub fn caly(&self, x:T)->T{(self.a*x+self.c)/-self.b}
	pub fn calx(&self, y:T)->T{(self.b*y+self.c)/-self.a}
	pub fn tan(&self)->T{-self.a/self.b}
	pub fn intersect(&self, rhs:&Self)->Result<Vec2<T>,LineErr>{
		let det = self.a*rhs.b-self.b*rhs.a;
		if det.is_zero(){
			if self.a*rhs.c==rhs.a*self.c {Err(LineErr::Same)}
			else {Err(LineErr::Parallel)}
		}else{
			Ok(Vec2{x:(self.b*rhs.c-rhs.b*self.c)/det, y:(self.c*rhs.a-rhs.c*self.a)/det})
		}
	}
}