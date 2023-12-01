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
	//example: https://codeforces.com/contest/1900/submission/234490943
	pub fn scc(&self) -> (Vec<Vec<usize>>, Vec<usize>) {
		let n = self.len();
		let mut scc = Vec::new();
		let mut sccid = vec![n; n];
		let mut ord = vec![n; n];
		let mut ordmin = vec![n; n];
		let mut ordid = 0;
		let mut stk = Vec::new();
		for i in 0..n {
			if ord[i] == n {
				self.dfs_tarjan(
					i,
					&mut scc,
					&mut sccid,
					&mut ord,
					&mut ordmin,
					&mut ordid,
					&mut stk,
				);
			}
		}
		(scc, sccid)
	}
	fn dfs_tarjan(
		&self,
		x: usize,
		scc: &mut Vec<Vec<usize>>,
		sccid: &mut Vec<usize>,
		ord: &mut Vec<usize>,
		ordmin: &mut Vec<usize>,
		ordid: &mut usize,
		stk: &mut Vec<usize>,
	) -> usize {
		stk.push(x);
		ord[x] = *ordid;
		ordmin[x] = *ordid;
		*ordid += 1;
		for y in self.adj[x].iter() {
			if ord[*y] == self.len() {
				//tree-edge
				ordmin[x] = ordmin[x].min(self.dfs_tarjan(*y, scc, sccid, ord, ordmin, ordid, stk));
			} else if sccid[*y] == self.len() {
				ordmin[x] = ordmin[x].min(ord[*y]);
			}
		}
		if ordmin[x] == ord[x] {
			scc.push(Vec::new());
			while let Some(y) = stk.pop() {
				scc.last_mut().unwrap().push(y);
				sccid[y] = scc.len() - 1;
				if y == x {
					break;
				}
			}
		}
		ordmin[x]
	}
}
