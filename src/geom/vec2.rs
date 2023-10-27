use std::ops::{Mul, Add, Neg, Sub};

use crate::math::structs::Field;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2<T>{
	pub x:T, pub y:T
}

impl<T> Vec2<T> where T:Field{
	pub fn dot(&self,rhs:&Self)->T{self.x*rhs.x+self.y*rhs.y}
	pub fn lensq(&self)->T{self.dot(self)}
	pub fn mul(&self,scale:T)->Self{Self{x:self.x*scale, y:self.y*scale}}
	pub fn rot90(&self)->Self{Self{x:self.y,y:-self.x}}
}
impl<T> Add for Vec2<T> where T:Add<Output=T>{
	type Output=Self;
	fn add(self, rhs: Self) -> Self::Output {Self{x:self.x+rhs.x, y:self.y+rhs.y}}
}
impl<T> Neg for Vec2<T> where T:Neg<Output=T>{
	type Output=Self;
	fn neg(self) -> Self::Output {Self{x:-self.x, y:-self.y}}
}
impl<T> Sub for Vec2<T> where T:Add<Output=T>+Neg<Output=T>{
	type Output=Self;
	fn sub(self, rhs: Self) -> Self::Output {self+-rhs}
}

pub fn ccw<T>(a:Vec2<T>, b:Vec2<T>, c:Vec2<T>)->T
where T:Copy+Mul<Output=T>+Add<Output=T>+Sub<Output=T>{
	a.x*b.y+b.x*c.y+c.x*a.y-a.x*c.y-b.x*a.y-c.x*b.y
}