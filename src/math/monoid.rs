pub trait Monoid{
	fn id()->Self;
	fn f(&self, rhs:Self)->Self;
}

// //monoid{i64,+}
// impl Monoid for i64{
// 	fn id()->Self {0}
// 	fn f(&self, rhs:Self)->Self {*self + rhs}
// }

// //monoid{i64,min}
// impl Monoid for i64{
// 	fn id()->Self {i64::MAX}
// 	fn f(&self, rhs:Self)->Self {min(*self,rhs)}
// }

// //monoid{i64,max}
// impl Monoid for i64{
// 	fn id()->Self {i64::MIN}
// 	fn f(&self, rhs:Self)->Self {max(*self,rhs)}
// }

// //monoid{(val,idx),min}
// impl Monoid for (usize,usize){
// 	fn id()->Self{(usize::MAX,usize::MAX)}
// 	fn f(&self,rhs:Self)->Self{min(*self,rhs)}
// }

// //monoid{(val,idx),max}
// impl Monoid for (usize,usize){
// 	fn id()->Self{(usize::MIN,usize::MIN)}
// 	fn f(&self,rhs:Self)->Self{max(*self,rhs)}
// }


pub trait MonoidLazy{
	fn idQ()->Self;
	fn idU()->Self;
	fn q(&self, rhs:Self)->Self;
	fn upd(&self, rhs:Self, cnt:i64)->Self;
	fn acc(&self, rhs:Self)->Self;
}
// //Query +, Update +
// impl MonoidLazy for i64{
// 	fn idQ()->Self {0}
// 	fn idU()->Self {0}
// 	fn q(&self, rhs:Self)->Self { *self+rhs }
// 	fn upd(&self, rhs:Self, cnt:i64)->Self { *self+rhs*cnt }
// 	fn acc(&self, rhs:Self)->Self { *self+rhs }
// }
// //Query max, Update +
// impl MonoidLazy for i64{
// 	fn idQ()->Self{0/*i64::MIN*/}
// 	fn idU()->Self{0}
// 	fn q(&self, rhs:Self)->Self{ *self.max(&rhs) }
// 	fn upd(&self, rhs:Self, cnt:i64)->Self{ *self+rhs }
// 	fn acc(&self, rhs:Self)->Self{ *self+rhs }
// }