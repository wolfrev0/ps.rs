use std::{cell::Cell, mem::swap};

pub struct UF {
	pub p: Vec<Cell<usize>>,
	pub g: Vec<Vec<usize>>,
}
impl UF {
	pub fn new(n: usize) -> UF {
		UF {
			p: (0..n).map(|x| Cell::new(x)).collect(),
			g: (0..n).map(|x| vec![x]).collect(),
		}
	}
	pub fn root(&self, x: usize) -> usize {
		let new_px = if self.p[x].get() == x {
			x
		} else {
			self.root(self.p[x].get())
		};
		self.p[x].set(new_px);
		new_px
	}
	pub fn union(&mut self, mut x: usize, mut y: usize) -> bool {
		x = self.root(x);
		y = self.root(y);
		if x == y {
			false
		} else {
			if self.g[x].len() < self.g[y].len() {
				swap(&mut x, &mut y);
			}
			self.p[y].set(x);
			let mut gy = Vec::new();
			swap(&mut self.g[y], &mut gy);
			self.g[x].extend(gy);
			true
		}
	}
	pub fn gsz(&self, x: usize) -> usize {
		self.g[self.root(x)].len()
	}
}
