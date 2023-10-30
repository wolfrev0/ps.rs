pub struct UD<I: Clone> {
	pub adj: Vec<Vec<(usize, I)>>,
}

impl<I: Clone> UD<I> {
	pub fn new(n: usize) -> Self {
		Self {
			adj: vec![Vec::new(); n],
		}
	}
	pub fn len(&self) -> usize {
		self.adj.len()
	}
}

impl UD<()> {
	pub fn add_edge(&mut self, from: usize, to: usize) {
		self.adj[from].push((to, ()));
	}
	pub fn reverse(&self) -> UD<()> {
		let mut ret = UD::<()>::new(self.len());
		for x in 0..self.len() {
			for (y, _) in self.adj[x].iter() {
				ret.add_edge(*y, x);
			}
		}
		ret
	}
}
