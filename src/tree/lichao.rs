use crate::{geom::line2::Line2, math::structs::{Zero, One, Field, Inf}};

trait LichaoType<T>{
	fn eval(&self, x:T)->T;
	fn id()->Self;
}
impl<T> LichaoType<T> for Line2<T> where T:Field+Inf{
	fn eval(&self, x:T)->T {self.caly(x)}
	fn id()->Self{Self{a:T::zero(),b:T::one(),c:T::inf()}}
}

struct Node<T>{
	v:T,l:usize,r:usize
}
pub struct Lichao<T, const XS:i64, const XE:i64>{
	a:Vec<Node<T>>,
}

impl<T, const XS:i64, const XE:i64> Lichao<T, XS, XE> where T:LichaoType<T>{
	pub fn new()->Lichao<T,XS,XE>{
		let mut ret = Lichao{a:Vec::new()};
		ret.alloc();
		ret
	}
	fn alloc(&mut self)->usize{
		self.a.push(Node{v:T::id(), l:0, r:0});
		self.a.len()-1
	}
}
