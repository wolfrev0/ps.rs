pub struct CoordCompress<T>{
	a:Vec<T>,
}
impl<T:Ord+Copy> CoordCompress<T>{
	pub fn new(mut a:Vec<T>)->CoordCompress<T>{a.sort();CoordCompress{a}}
	pub fn zip(&self,x:T)->usize{
		match self.a.binary_search(&x){
			Ok(idx) => idx,
			Err(idx) => idx,
		}
	}
	pub fn unzip(&self,x:usize)->T{self.a[x]}
}