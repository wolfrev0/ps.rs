use crate::util::permu_inv;

use super::tree::Tree;

pub struct HLD {
	pub root: usize,
	pub p: Vec<usize>,       //parent of vertex i
	pub ch: Vec<Vec<usize>>, //children of vertex i
	pub sz: Vec<usize>,      //size of subtree i
	pub dpt: Vec<usize>,     //depth of vertex i
	pub chain: Vec<usize>,   //chain of vertex i
	pub top: Vec<usize>,     //top vertex of the chain i
	pub idx: Vec<usize>,     //preorder index of vertex i
}
impl HLD {
	pub fn len(&self) -> usize {
		self.p.len()
	}
	pub fn new(tr: Tree, root: usize) -> HLD {
		let mut ret = HLD {
			root: root,
			p: vec![usize::MAX; tr.len()],
			ch: vec![Vec::new(); tr.len()],
			sz: vec![0; tr.len()],
			dpt: vec![0; tr.len()],
			chain: vec![usize::MAX; tr.len()],
			top: vec![root; 1],
			idx: Vec::new(),
		};
		ret.dfs_init(&tr, root, root);
		ret.dfs_hld(root);
		ret.idx = permu_inv(&ret.pre_order());
		ret
	}
	fn dfs_init(&mut self, tr: &Tree, x: usize, px: usize) {
		self.sz[x] = 1;
		for y in tr.g[x].iter() {
			if *y != px {
				self.dpt[*y] = self.dpt[x] + 1;
				self.p[*y] = x;
				//y를 x의 값으로 채우는건 재귀 이전에
				self.dfs_init(tr, *y, x);
				//x를 y의 값으로 채우는건 재귀 이후에
				self.ch[x].push(*y);
				self.sz[x] += self.sz[*y];
			}
		}
		// make largest child subtree to be first element for HLD
		for i in 1..self.ch[x].len() {
			if self.sz[self.ch[x][0]] < self.sz[self.ch[x][i]] {
				self.ch[x].swap(0, i)
			}
		}
	}
	fn dfs_hld(&mut self, x: usize) {
		self.chain[x] = self.top.len() - 1;
		for y in self.ch[x].to_owned() {
			if y != *self.ch[x].first().unwrap() {
				self.top.push(y);
			}
			self.dfs_hld(y);
		}
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

	//return = (LCA, a to LCA, LCA to b)
	//LCA는 a to LCA와 LCA to b에 포함되지 않음.
	//Non commutative element에 대한 쿼리시 다음과 같이 뒤집혀있는걸 잘 처리해줘야함.
	//a to LCA는 전체 순서는 맞지만 개별 구간이 뒤집혀있음. 개별구간.rev() 사용
	//LCA to b는 개별 구간은 맞지만 전체 순서가 뒤집혀있음. reverse(LCA to b) 사용
	pub fn query(
		&self,
		mut a: usize,
		mut b: usize,
	) -> (usize, Vec<(usize, usize)>, Vec<(usize, usize)>) {
		let lca;
		let mut reta = Vec::new();
		let mut retb = Vec::new();
		while self.chain[a] != self.chain[b] {
			if self.dpt[self.top[self.chain[a]]] > self.dpt[self.top[self.chain[b]]] {
				reta.push((self.idx[self.top[self.chain[a]]], self.idx[a] + 1));
				a = self.p[self.top[self.chain[a]]];
			} else {
				retb.push((self.idx[self.top[self.chain[b]]], self.idx[b] + 1));
				b = self.p[self.top[self.chain[b]]];
			}
		}
		if self.dpt[a] > self.dpt[b] {
			lca = b;
			reta.push((self.idx[b] + 1, self.idx[a] + 1));
		} else {
			lca = a;
			retb.push((self.idx[a] + 1, self.idx[b] + 1));
		}
		(lca, reta, retb)
	}

	pub fn dist(&self, a: usize, b: usize) -> usize {
		let (lca, _, _) = self.query(a, b);
		self.dpt[a] + self.dpt[b] - self.dpt[lca] * 2
	}
}
