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