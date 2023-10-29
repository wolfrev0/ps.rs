use std::collections::BTreeMap;

pub struct Multiset<T> {
	pub a: BTreeMap<T, usize>,
	cnt: usize,
}
impl<T: Clone + Ord> Multiset<T> {
	pub fn new() -> Self {
		Self {
			a: BTreeMap::<T, usize>::new(),
			cnt: 0,
		}
	}
	pub fn insert(&mut self, x: &T) {
		match self.a.get_mut(x) {
			Some(y) => *y += 1,
			None => {
				self.a.insert(x.clone(), 1);
			}
		}
		self.cnt += 1;
	}
	pub fn remove(&mut self, x: &T) {
		match self.a.get_mut(x) {
			Some(y) => {
				*y -= 1;
				if *y == 0 {
					self.a.remove(x);
				}
			}
			None => {
				panic!("No element");
			}
		}
		self.cnt -= 1;
	}
	pub fn contains(&self, x: &T) -> bool {
		self.a.contains_key(x)
	}
	pub fn len(&self) -> usize {
		self.cnt
	}
}
