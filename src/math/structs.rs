use std::ops::{Add, Mul, Neg, Sub, Div, Rem};

pub trait Zero<T>{
	fn zero()->T;
	fn is_zero(&self)->bool;
}
impl Zero<i32> for i32{
	fn zero()->i32{0}
	fn is_zero(&self)->bool{*self==0}
}
impl Zero<i64> for i64{
	fn zero()->i64{0}
	fn is_zero(&self)->bool{*self==0}
}
impl Zero<f32> for f32{
	fn zero()->f32{0.0}
	fn is_zero(&self)->bool{*self==0.}
}
impl Zero<f64> for f64{
	fn zero()->f64{0.0}
	fn is_zero(&self)->bool{*self==0.}
}

pub trait One<T>{
	fn one()->T;
	fn is_one(&self)->bool;
}
impl One<i32> for i32{
	fn one()->i32{1}
	fn is_one(&self)->bool{*self==1}
}
impl One<i64> for i64{
	fn one()->i64{1}
	fn is_one(&self)->bool{*self==1}
}
impl One<f32> for f32{
	fn one()->f32{1.0}
	fn is_one(&self)->bool{*self==1.}
}
impl One<f64> for f64{
	fn one()->f64{1.0}
	fn is_one(&self)->bool{*self==1.}
}

pub trait Group<T>
{}//TODO

pub trait Ring<T>
where T:Copy+PartialEq+PartialOrd+Zero<T>+One<T>
+Add<Output=T>+Mul<Output=T>+Neg<Output=T>+Sub<Output=T>{
}
impl Ring<i32> for i32{}
impl Ring<i64> for i64{}
impl Ring<f32> for f32{}
impl Ring<f64> for f64{}

pub trait Field<T>
where T:Copy+PartialEq+PartialOrd+Zero<T>+One<T>
+Add<Output=T>+Mul<Output=T>+Neg<Output=T>+Sub<Output=T>+Ring<T>
+Div<Output=T>+Rem<Output=T>{}
impl Field<i32> for i32{}
impl Field<i64> for i64{}
impl Field<f32> for f32{}
impl Field<f64> for f64{}