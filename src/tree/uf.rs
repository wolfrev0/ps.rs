use std::mem::swap;

pub struct UF {
	pub p: Vec<usize>,
	pub g: Vec<Vec<usize>>,
}
impl UF {
	pub fn new(n: usize) -> UF {
		UF {
			p: (0..n).collect(),
			g: (0..n).map(|x| vec![x]).collect(),
		}
	}
	pub fn root(&mut self, x: usize) -> usize {
		self.p[x] = if self.p[x] == x {
			x
		} else {
			self.root(self.p[x])
		};
		self.p[x]
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
			self.p[y] = x;
			let mut gy = Vec::new();
			swap(&mut self.g[y], &mut gy);
			self.g[x].extend(gy);
			true
		}
	}
	pub fn gsz(&self, x: usize) -> usize {
		self.g[x].len()
	}
}
