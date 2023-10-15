use crate::math::monoid::Monoid;

pub struct Seg<T>{
	a:Vec<T>,
}
impl<T:Monoid+Clone> Seg<T>{
	pub fn new(n:usize)->Seg<T>{Seg{a:vec![T::id();n*2]}}
	pub fn size(&self)->usize{self.a.len()/2}
	pub fn q(&self,mut s:usize,mut e:usize)->T{
		let (mut rs,mut re)=(T::id(), T::id());
		s+=self.size();
		e+=self.size();
		while s<e{
			if s&1==1{rs=rs.f(self.a[s].clone()); s+=1}
			if e&1==1{e-=1; re=self.a[e].f(re)}
			s>>=1;
			e>>=1;
		}
		rs.f(re)
	}
	pub fn upd(&mut self, mut idx:usize, val:T){
		idx+=self.size();
		self.a[idx]=val;
		idx>>=1;
		while idx>0{
			self.a[idx] = self.a[idx<<1].f(self.a[idx<<1|1].clone());
			idx>>=1;
		}
	}
}

#[cfg(test)]
mod tests {
	use std::cmp::min;
	use crate::{tree::seg::Seg, math::monoid::Monoid};
	#[test]
	fn test_i64mul() {
		impl Monoid for i32{
			fn id()->Self{1}
			fn f(&self,rhs:Self)->Self{self*rhs}
		}
		let mut st = Seg::<i32>::new(5);
		assert_eq!(st.q(0,5), 1);
		st.upd(3,7);
		st.upd(4,3);
		assert_eq!(st.q(2,3), 1);
		assert_eq!(st.q(3,4), 7);
		assert_eq!(st.q(4,5), 3);
		assert_eq!(st.q(2,5), 21);
	}
	#[test]
	fn test_struct() {
		#[derive(Clone)]pub struct MAdd<T>{pub x:T}
		impl Monoid for MAdd<i32>{
			fn id()->Self{Self{x:0}}
			fn f(&self,rhs:Self)->Self{Self{x:self.x+rhs.x}}
		}
		let mut st = Seg::<MAdd::<i32>>::new(5);
		assert_eq!(st.q(0,5).x, 0);
		st.upd(3,MAdd::<i32>{x:7});
		st.upd(4,MAdd::<i32>{x:3});
		assert_eq!(st.q(2,3).x, 0);
		assert_eq!(st.q(3,4).x, 7);
		assert_eq!(st.q(4,5).x, 3);
		assert_eq!(st.q(2,5).x, 10);
	}
	#[test]
	fn test_pairmin() {
		impl Monoid for (i32,usize){
			fn id()->Self{(i32::MAX,usize::MAX)}
			fn f(&self,rhs:Self)->Self{min(*self,rhs)}
		}
		let mut st = Seg::<(i32,usize)>::new(5);
		assert_eq!(st.q(0,5), (i32::MAX,usize::MAX));
		st.upd(3,(7,3));
		st.upd(4,(3,4));
		assert_eq!(st.q(2,3), (i32::MAX,usize::MAX));
		assert_eq!(st.q(3,4), (7,3));
		assert_eq!(st.q(4,5), (3,4));
		assert_eq!(st.q(2,5), (3,4));
	}
}