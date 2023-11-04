use crate::tree::uf::UF;

pub struct AdjListUD {
	pub adj: Vec<Vec<usize>>,
}
impl AdjListUD {
	pub fn new(n: usize) -> Self {
		Self {
			adj: vec![Vec::new(); n],
		}
	}
	pub fn len(&self) -> usize {
		self.adj.len()
	}
	pub fn add_edge(&mut self, from: usize, to: usize) {
		self.adj[from].push(to);
	}
	pub fn reverse(&self) -> Self {
		let mut ret = Self::new(self.len());
		for x in 0..self.len() {
			for y in self.adj[x].iter() {
				ret.add_edge(*y, x);
			}
		}
		ret
	}
	pub fn is_bipartite(&self) -> bool {
		let mut uf = UF::new(self.len() * 2);
		for x in 0..self.len() {
			for y in self.adj[x].iter() {
				uf.union(x, y + self.len());
				uf.union(x + self.len(), *y);
			}
		}
		for i in 0..self.len() {
			if uf.root(i) == uf.root(i + self.len()) {
				return false;
			}
		}
		return true;
	}
}
