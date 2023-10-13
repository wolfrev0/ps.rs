pub trait Monoid{
	fn id()->Self;
	fn f(&self, rhs:Self)->Self;
}
#[derive(Clone)]pub struct MAdd<T>{pub x:T}
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

impl Monoid for MAdd<i32>{
	fn id()->Self{Self{x:0}}
	fn f(&self,rhs:Self)->Self{Self{x:self.x+rhs.x}}
}
impl Monoid for MAdd<i64>{
	fn id()->Self{Self{x:0}}
	fn f(&self,rhs:Self)->Self{Self{x:self.x+rhs.x}}
}

#[cfg(test)]
mod tests {
    use crate::seg::{Seg, MAdd};

	#[test]
	fn test_negi_0() {
		let mut st = Seg::<MAdd::<i32>>::new(5);
		assert_eq!(st.q(0,5).x, 0);
		st.upd(3,MAdd::<i32>{x:7});
		st.upd(4,MAdd::<i32>{x:3});
		assert_eq!(st.q(2,3).x, 0);
		assert_eq!(st.q(3,4).x, 7);
		assert_eq!(st.q(4,5).x, 3);
		assert_eq!(st.q(2,5).x, 10);
	}
}