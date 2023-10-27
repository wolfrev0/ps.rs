use std::ops::{Add, Mul, Neg, Sub, Rem, Div};

use crate::math::structs::{Zero, One};

use super::vec2::{Vec2, ccw};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Polygon2<T>{
	pub a:Vec<Vec2<T>>,
}
impl<T> Polygon2<T>
where T:Copy+PartialEq+Zero<T>+One<T>
+Add<Output=T>+Mul<Output=T>+Neg<Output=T>+Sub<Output=T>
+Div<Output=T>+Rem<Output=T>{
	pub fn area_doubled(&self)->T{
		let mut ret = T::zero();
		for i in 1..self.a.len()-1{
			ret = ret + ccw(self.a[i-1], self.a[i], self.a[i+1])
		}
		ret
	}
}

pub enum PointOnPolygon2{
	INSIDE,
	EDGE,
	OUTSIDE,
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolygonConvex2<T>{
	pub a:Vec<Vec2<T>>,
}
impl<T> PolygonConvex2<T>
where T:Copy+PartialEq+Zero<T>+One<T>
+Add<Output=T>+Mul<Output=T>+Neg<Output=T>+Sub<Output=T>
+Div<Output=T>+Rem<Output=T>{
	pub fn normalized(&self)->PolygonConvex2<T>{
		//TODO
		assert!(false);
		self.clone()
	}
	pub fn contains(v:Vec2<T>)->PointOnPolygon2{
		assert!(false);
    let _ = v;
		PointOnPolygon2::INSIDE
	}
}