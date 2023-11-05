pub struct Tree {
	pub g: Vec<Vec<usize>>,
}
impl Tree {
	pub fn new(n: usize) -> Tree {
		Tree {
			g: vec![Vec::new(); n],
		}
	}

	pub fn len(&self) -> usize {
		self.g.len()
	}

	pub fn add_edge(&mut self, x: usize, y: usize) {
		self.g[x].push(y);
		self.g[y].push(x);
	}

	pub fn centroid_tree(&self) -> TreeRooted {
		let mut dead = vec![false; self.len()];
		let mut sizes = vec![0; self.len()];
		self.recalc_sizes(0, 0, &dead, &mut sizes);
		let cen = self.find_centroid(0, 0, &mut dead, sizes[0], &mut sizes);
		TreeRooted::new(
			self.centroid_decompose(cen, &mut dead, &mut sizes, Tree::new(self.len())),
			cen,
		)
	}
	fn recalc_sizes(&self, x: usize, p: usize, dead: &Vec<bool>, mut sizes: &mut Vec<usize>) {
		sizes[x] = 1;
		for y in self.g[x].iter() {
			if *y == p || dead[*y] {
				continue;
			}
			self.recalc_sizes(*y, x, &dead, &mut sizes);
			sizes[x] += sizes[*y];
		}
	}
	fn find_centroid(
		&self,
		x: usize,
		p: usize,
		dead: &Vec<bool>,
		sz: usize,
		sizes: &Vec<usize>,
	) -> usize {
		for y in self.g[x].iter() {
			if *y == p || dead[*y] {
				continue;
			}
			if sizes[*y] * 2 > sz {
				return self.find_centroid(*y, x, &dead, sz, sizes);
			}
		}
		x
	}
	fn centroid_decompose(
		&self,
		x: usize,
		mut dead: &mut Vec<bool>,
		sizes: &mut Vec<usize>,
		mut ctr: Tree,
	) -> Tree {
		dead[x] = true;
		for y in self.g[x].iter() {
			if dead[*y] {
				continue;
			}
			self.recalc_sizes(*y, x, &dead, sizes);
			let ceny = self.find_centroid(*y, x, &mut dead, sizes[*y], sizes);
			ctr.add_edge(ceny, x);
			ctr = self.centroid_decompose(ceny, &mut dead, sizes, ctr)
		}
		ctr
	}

	//TODO: Tree Compress(https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-2/problems/C)
}

pub struct TreeRooted {
	pub root: usize,
	pub p: Vec<usize>,
	pub ch: Vec<Vec<usize>>,
	pub sz: Vec<usize>,
}
impl TreeRooted {
	pub fn len(&self) -> usize {
		self.p.len()
	}
	pub fn new(tr: Tree, root: usize) -> TreeRooted {
		fn init_dfs(mut s: TreeRooted, tr: &Tree, x: usize, px: usize) -> TreeRooted {
			for y in tr.g[x].iter() {
				if *y != px {
					s = init_dfs(s, tr, *y, x);
					s.p[*y] = x;
					s.ch[x].push(*y);
					s.sz[x] += s.sz[*y];
				}
			}
			s.sz[x] += 1;
			// make largest child subtree to be first element.
			// It's useful When doing HLD.
			for i in 1..s.ch[x].len() {
				if s.sz[s.ch[x][0]] < s.sz[s.ch[x][i]] {
					s.ch[x].swap(0, i)
				}
			}
			s
		}
		init_dfs(
			TreeRooted {
				root: root,
				p: vec![0; tr.len()],
				ch: vec![Vec::new(); tr.len()],
				sz: vec![0; tr.len()],
			},
			&tr,
			root,
			root,
		)
	}
	pub fn pre_order(&self) -> Vec<usize> {
		fn dfs(ch: &Vec<Vec<usize>>, x: usize, res: &mut Vec<usize>) {
			res.push(x);
			for y in ch[x].iter() {
				dfs(ch, *y, res);
			}
		}
		let mut ret = Vec::new();
		dfs(&self.ch, self.root, &mut ret);
		ret
	}
}
