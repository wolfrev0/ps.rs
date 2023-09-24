use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct Vex<T>(Vec<T>);
impl<T>Vex<T>{
	pub const fn new()->Self{Vex{0:Vec::new()}}
	pub fn len(&self)->i32{(self as &Vec<T>).len() as i32}
}
impl<T>Deref for Vex<T>{
	type Target=Vec<T>;
	fn deref(&self)->&Vec<T> {&self.0}
}
impl<T>DerefMut for Vex<T>{
	fn deref_mut(&mut self)->&mut Vec<T>{&mut self.0}
}
impl<T>Index<i32> for Vex<T>{
	type Output = T;
	fn index(&self, i:i32)->&T{
		&self.0[if i<0 {(self.len() as i32+i) as usize} else {i as usize}]
	}
}
impl<T>IndexMut<i32> for Vex<T>{
	fn index_mut(&mut self, i:i32)->&mut T{
		let len=self.len();
		&mut self.0[if i<0 {(len as i32+i) as usize} else {i as usize}]
	}
}
impl<T>FromIterator<T> for Vex<T>{
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let mut c = Vex::new();for i in iter {c.push(i);};c
	}
}