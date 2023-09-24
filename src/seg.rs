pub struct Seg<T>{
	n:usize,
	a:Vec<T>,
}
pub trait Monoid:Default+Copy{
	fn op(&self, rhs:Self)->Self;
	fn upd(&mut self, rhs:Self)->Self;
}
impl<T:Monoid> Seg<T>{
	pub fn new(n:usize)->Seg<T>{Seg{n,a:vec![T::default();n*4]}}
	pub fn q(&self,s:usize,e:usize)->T{self._q(s,e,0,self.n,1)}
	pub fn upd(&mut self,idx:usize,val:T){self._upd(idx,val,0,self.n,1);}
	pub fn _q(&self,s:usize,e:usize,cs:usize,ce:usize,ai:usize)->T{
		if ce<=s || e<=cs {
			return T::default()
		}else if s<=cs && ce<=e {
			self.a[ai]
		}else{
			let cm=(cs+ce)/2;
			self._q(s,e,cs,cm,ai*2).op(self._q(s,e,cm,ce,ai*2+1))
		}
	}
	pub fn _upd(&mut self,idx:usize,val:T,cs:usize,ce:usize,ai:usize)->T{
		if ce<=idx || idx+1<=cs {
			
		} else if idx<=cs && ce<=idx+1 {
			self.a[ai].upd(val);
		} else {
			let cm=(cs+ce)/2;
			self.a[ai]=self._upd(idx,val,cs,cm,ai*2).op(self._upd(idx,val,cm,ce,ai*2+1));
		}
		self.a[ai]
	}
}
impl Monoid for i64{
	fn op(&self,rhs:Self)->Self{
		self+rhs
	}
	fn upd(&mut self,rhs:Self)->Self{
		*self+=rhs; *self
	}
}