use super::hld::HLD;

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

	pub fn centroid_tree(&self) -> HLD {
		let mut dead = vec![false; self.len()];
		let mut sizes = vec![0; self.len()];
		self.recalc_sizes(0, 0, &dead, &mut sizes);
		let cen = self.find_centroid(0, 0, &mut dead, sizes[0], &mut sizes);
		HLD::new(
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
