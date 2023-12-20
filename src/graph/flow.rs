use std::{cmp::Reverse, collections::VecDeque, mem::swap};

use crate::math::structs::{inf::Inf, zero::Zero};

use super::{ud::UD, wd::WD};

#[derive(Clone)]
pub struct FlowInfo {
	pub resi: usize, //residual index
	pub cap: usize,  //capacity
}
impl UD<FlowInfo> {
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
				cap: cap,
			},
		));
		self.adj[to].push((
			from,
			FlowInfo {
				resi: resi_to,
				cap: 0,
			},
		))
	}
	#[deprecated(
		since = "0.1.0",
		note = "please use `.dinic()` instead for better performance."
	)]
	pub fn ford_fulkerson(&mut self, src: usize, snk: usize) -> usize {
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
		&mut self,
		snk: usize,
		x: usize,
		flow: usize,
		vis: &mut Vec<bool>,
	) -> usize {
		if x == snk {
			return flow;
		}
		vis[x] = true;
		for i in 0..self.adj[x].len() {
			let (y, FlowInfo { resi, cap }) = self.adj[x][i];
			if !vis[y] && cap > 0 {
				let flow_cur = self.ford_fulkerson_dfs(snk, y, flow.min(cap), vis);
				if flow_cur > 0 {
					self.adj[x][i].1.cap -= flow_cur;
					self.adj[y][resi].1.cap += flow_cur;
					return flow_cur;
				}
			}
		}
		0
	}

	pub fn dinic(&mut self, src: usize, snk: usize) -> usize {
		let mut ret = 0;
		loop {
			let dist = self.dinic_bfs(src);
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
	fn dinic_bfs(&self, src: usize) -> Vec<usize> {
		let mut dist = vec![usize::MAX / 2; self.len()];
		let mut q = VecDeque::new();
		dist[src] = 0;
		q.push_back(src);
		while q.len() > 0 {
			let x = q.pop_front().unwrap();
			for (y, FlowInfo { resi: _, cap }) in self.adj[x].iter() {
				if *cap > 0 && dist[*y] > dist[x] + 1 {
					dist[*y] = dist[x] + 1;
					q.push_back(*y);
				}
			}
		}
		dist
	}
	//snk에서 역방향으로 src에 도달가능한 간선만 사용하면 더 빨라질수도 있다고 함
	fn dinic_dfs(
		&mut self,
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
				let (y, FlowInfo { resi, cap }) = self.adj[x][idx_base[x]];
				if dist[y] == dist[x] + 1 && cap > 0 {
					let flow_cur = self.dinic_dfs(snk, dist, y, idx_base, flow.min(cap));
					if flow_cur > 0 {
						self.adj[x][idx_base[x]].1.cap -= flow_cur;
						self.adj[y][resi].1.cap += flow_cur;
						return flow_cur;
					}
				}
				idx_base[x] += 1;
			}
			0
		}
	}

	pub fn cut(&mut self, src: usize, snk: usize) -> (usize, Vec<usize>, Vec<usize>) {
		let f = self.dinic(src, snk);
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
		(f, s, t)
	}
	fn cut_dfs(&self, x: usize, vis: &mut Vec<bool>) {
		vis[x] = true;
		for (y, FlowInfo { resi: _, cap }) in self.adj[x].iter() {
			if !vis[*y] && *cap > 0 {
				self.cut_dfs(*y, vis);
			}
		}
	}
}

impl WD<i64, FlowInfo> {
	pub fn add_edge(&mut self, from: usize, to: usize, cost: i64, cap: usize) {
		let resi_from = self.adj[to].len();
		let resi_to = self.adj[from].len();
		self.adj[from].push((
			to,
			cost,
			FlowInfo {
				resi: resi_from,
				cap: cap,
			},
		));
		self.adj[to].push((
			from,
			-cost,
			FlowInfo {
				resi: resi_to,
				cap: 0,
			},
		))
	}
	pub fn min_cost_flow(&mut self, src: usize, snk: usize, must_maxflow: bool) -> (i64, usize) {
		let (mut flow, mut cost) = (0, 0);
		let n = self.len();
		loop {
			//SPFA
			let mut d = vec![(i64::inf(), 0, 0, 0); n]; //dist, flow, from, to_idx
			let mut p = Vec::new();
			let mut q = Vec::new();
			d[src] = (0, usize::MAX, 0, 0);
			p.push(src);
			for _epoch in 0..n {
				for i in p.iter() {
					for (ji, (j, w, FlowInfo { cap, resi: _ })) in self.adj[*i].iter().enumerate() {
						if *cap > 0 && d[*j].0 > d[*i].0 + *w {
							d[*j] = (d[*i].0 + *w, d[*i].1.min(*cap), *i, ji);
							q.push(*j);
						}
					}
				}
				swap(&mut p, &mut q);
				q.clear();
			}
			assert!(p.is_empty()); //if not, negative cycle exist

			let flow_cur = d[snk].1;
			if flow_cur.is_zero() {
				break;
			}
			let mut cost_cur = 0;
			let mut y = snk;
			while y != src {
				let x = d[y].2;
				let yi = d[y].3;
				let xi = self.adj[x][yi].2.resi;
				self.adj[x][yi].2.cap -= flow_cur;
				self.adj[y][xi].2.cap += flow_cur;
				cost_cur += flow_cur as i64 * self.adj[x][yi].1;
				y = x;
			}
			if !must_maxflow && cost_cur < 0 {
				break;
			}
			flow += flow_cur;
			cost += cost_cur;
		}
		(cost, flow)
	}
}
