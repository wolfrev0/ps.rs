// pub mod hash;

use std::collections::BTreeMap;

pub static DIR4: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub static DIR8: [(i32, i32); 8] = [
	(0, 1),
	(-1, 1),
	(-1, 0),
	(-1, -1),
	(0, -1),
	(1, -1),
	(1, 0),
	(1, 1),
];
pub static ENDL: char = '\n';

pub fn classify<T>(a: &Vec<T>) -> BTreeMap<T, Vec<usize>>
where
	T: Clone + Ord,
{
	let mut ret = BTreeMap::new();
	for i in 0..a.len() {
		if ret.contains_key(&a[i]) == false {
			ret.insert(a[i].clone(), Vec::new());
		}
		ret.get_mut(&a[i]).unwrap().push(i)
	}
	ret
}

pub fn permu_inv<T>(a: &Vec<T>) -> Vec<T>
where
	T: Copy,
	usize: From<T>,
	T: From<usize>,
{
	let mut b = vec![T::from(0); a.len()];
	for i in 0..a.len() {
		b[usize::from(a[i])] = T::from(i);
	}
	b
}

pub struct CoordCompress<T> {
	a: Vec<T>,
}
impl<T: Ord + Copy> CoordCompress<T> {
	pub fn new(mut a: Vec<T>) -> CoordCompress<T> {
		a.sort();
		CoordCompress { a }
	}
	pub fn zip(&self, x: T) -> usize {
		match self.a.binary_search(&x) {
			Ok(idx) => idx,
			Err(idx) => idx,
		}
	}
	pub fn unzip(&self, x: usize) -> T {
		self.a[x]
	}
}