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

//return (root, tree)
//https://en.wikipedia.org/wiki/Cartesian_tree
pub fn build_cartesian(a: Vec<usize>) -> (usize, Tree) {
	let n = a.len();
	let mut l = vec![None; n];
	let mut r = vec![None; n];
	let mut stk = Vec::new();
	for i in 0..n {
		while stk.len() > 0 && a[*stk.last().unwrap()] > a[i] {
			stk.pop();
		}
		l[i] = stk.last().cloned();
		stk.push(i);
	}
	stk.clear();
	for i in (0..n).rev() {
		while stk.len() > 0 && a[*stk.last().unwrap()] > a[i] {
			stk.pop();
		}
		r[i] = stk.last().cloned();
		stk.push(i);
	}
	let mut tr = Tree::new(n);
	let mut root = 0;
	for i in 0..n {
		if i == root {
			if let Some(j) = l[i] {
				tr.add_edge(i, j);
			}
			if let Some(j) = r[i] {
				tr.add_edge(i, j);
			}
		} else {
			match (l[i], r[i]) {
				(None, None) => {}
				(None, Some(ri)) => tr.add_edge(i, ri),
				(Some(li), None) => tr.add_edge(i, li),
				(Some(li), Some(ri)) => tr.add_edge(i, if a[li] < a[ri] { ri } else { li }),
			}
		}
		if a[root] > a[i] {
			root = i;
		}
	}
	(root, tr)
}
mod test {
	#[test]
	fn test() {
		use crate::tree::tree::build_cartesian;
		use std::collections::BTreeSet;
		let (root, tr) = build_cartesian([9, 3, 7, 1, 8, 12, 10, 20, 15, 18, 5].to_vec());
		assert!(root == 3);
		assert!(BTreeSet::from_iter(tr.g[0].iter()) == BTreeSet::from_iter([1].iter()));
		assert!(BTreeSet::from_iter(tr.g[1].iter()) == BTreeSet::from_iter([3, 2, 0].iter()));
		assert!(BTreeSet::from_iter(tr.g[2].iter()) == BTreeSet::from_iter([1].iter()));
		assert!(BTreeSet::from_iter(tr.g[3].iter()) == BTreeSet::from_iter([1, 10].iter()));
		assert!(BTreeSet::from_iter(tr.g[4].iter()) == BTreeSet::from_iter([10, 6].iter()));
	}
}
