use std::{cell::Cell, clone, collections::VecDeque, ops::Add};

use crate::math::structs::{Inf, Zero};

use super::{ud::AdjListUD, wd::WD};

#[derive(Clone)]
pub struct FlowInfo {
	pub resi: usize,      //residual index
	pub cap: Cell<usize>, //capacity
}
pub struct Flow {
	pub adj: Vec<Vec<(usize, FlowInfo)>>,
}
impl Flow {
	pub fn new(n: usize) -> Self {
		Self {
			adj: vec![Vec::new(); n],
		}
	}
	pub fn len(&self) -> usize {
		self.adj.len()
	}
	pub fn add_edge(&mut self, from: usize, to: usize, cap: usize) {
		let resi_from = self.adj[to].len();
		let resi_to = self.adj[from].len();
		self.adj[from].push((
			to,
			FlowInfo {
				resi: resi_from,
				cap: Cell::new(cap),
			},
		));
		self.adj[to].push((
			from,
			FlowInfo {
				resi: resi_to,
				cap: Cell::new(0),
			},
		))
	}
	#[deprecated(
		since = "0.1.0",
		note = "please use `.dinic()` instead for better performance."
	)]
	pub fn ford_fulkerson(&self, src: usize, snk: usize) -> usize {
		let mut ret = 0;
		loop {
			let flow = self.ford_fulkerson_dfs(snk, src, usize::MAX, &mut vec![false; self.len()]);
			if flow == 0 {
				break;
			}
			ret += flow;
		}
		ret
	}
	fn ford_fulkerson_dfs(
		self: &Self,
		snk: usize,
		x: usize,
		flow: usize,
		vis: &mut Vec<bool>,
	) -> usize {
		if x == snk {
			return flow;
		}
		vis[x] = true;
		for (y, FlowInfo { resi, cap }) in self.adj[x].iter() {
			if !vis[*y] && cap.get() > 0 {
				let flow_cur = self.ford_fulkerson_dfs(snk, *y, flow.min(cap.get()), vis);
				if flow_cur > 0 {
					cap.set(cap.get() - flow_cur);
					let cap_resi = &self.adj[*y][*resi].1.cap;
					cap_resi.set(cap_resi.get() + flow_cur);
					return flow_cur;
				}
			}
		}
		0
	}

	pub fn dinic(&self, src: usize, snk: usize) -> usize {
		let mut ret = 0;
		loop {
			let dist = self.dinic_bfs(src, snk);
			let mut idx_base = vec![0; self.len()];
			let mut flow = 0;
			loop {
				let flow_cur = self.dinic_dfs(snk, &dist, src, &mut idx_base, usize::MAX);
				if flow_cur == 0 {
					break;
				}
				flow += flow_cur;
			}
			if flow == 0 {
				break;
			}
			ret += flow;
		}
		ret
	}
	fn dinic_bfs(&self, src: usize, snk: usize) -> Vec<usize> {
		let mut dist = vec![usize::MAX / 2; self.len()];
		let mut q = VecDeque::new();
		dist[src] = 0;
		q.push_back(src);
		while q.len() > 0 {
			let x = q.pop_front().unwrap();
			for (y, FlowInfo { resi: _, cap }) in self.adj[x].iter() {
				if cap.get() > 0 && dist[*y] > dist[x] + 1 {
					dist[*y] = dist[x] + 1;
					q.push_back(*y);
				}
			}
		}
		dist
	}
	//snk에서 역방향으로 src에 도달가능한 간선만 사용하면 더 빨라질수도 있다고 함
	fn dinic_dfs(
		&self,
		snk: usize,
		dist: &Vec<usize>,
		x: usize,
		idx_base: &mut Vec<usize>,
		flow: usize,
	) -> usize {
		if x == snk {
			flow
		} else {
			while idx_base[x] < self.adj[x].len() {
				let (y, FlowInfo { resi, cap }) = &self.adj[x][idx_base[x]];
				if dist[*y] == dist[x] + 1 && cap.get() > 0 {
					let flow_cur = self.dinic_dfs(snk, dist, *y, idx_base, flow.min(cap.get()));
					if flow_cur > 0 {
						cap.set(cap.get() - flow_cur);
						let cap_resi = &self.adj[*y][*resi].1.cap;
						cap_resi.set(cap_resi.get() + flow_cur);
						return flow_cur;
					}
				}
				idx_base[x] += 1;
			}
			0
		}
	}

	pub fn cut(&self, src: usize, snk: usize) -> (Vec<usize>, Vec<usize>) {
		self.dinic(src, snk);
		let mut vis = vec![false; self.len()];
		self.cut_dfs(src, &mut vis);
		let mut s = Vec::new();
		let mut t = Vec::new();
		for i in 0..self.len() {
			if vis[i] {
				s.push(i)
			} else {
				t.push(i)
			}
		}
		(s, t)
	}
	fn cut_dfs(&self, x: usize, vis: &mut Vec<bool>) {
		vis[x] = true;
		for (y, FlowInfo { resi: _, cap }) in self.adj[x].iter() {
			if !vis[*y] && cap.get() > 0 {
				self.cut_dfs(*y, vis);
			}
		}
	}
}

impl<W: Copy + Add<Output = W> + Inf + Zero + Ord> WD<W, FlowInfo> {
	pub fn add_edge(&mut self, from: usize, to: usize, cost: W, cap: usize) {
		let resi_from = self.adj[to].len();
		let resi_to = self.adj[from].len();
		self.adj[from].push((
			to,
			cost,
			FlowInfo {
				resi: resi_from,
				cap: Cell::new(cap),
			},
		));
		self.adj[to].push((
			from,
			cost,
			FlowInfo {
				resi: resi_to,
				cap: Cell::new(0),
			},
		))
	}
	pub fn min_cost_flow(&mut self, src: usize, snk: usize, flow_bound: usize) -> usize {
		panic!("TODO");
	}
}
