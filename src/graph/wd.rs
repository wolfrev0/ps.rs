use std::{cmp::Reverse, collections::BinaryHeap, mem::swap, ops::Add};

use crate::math::structs::{inf::Inf, zero::Zero};

pub struct WD<W: Copy + Add<Output = W> + Inf + Zero + Ord, I: Clone> {
	//W: 거리타입, I: 엣지 부가정보
	pub adj: Vec<Vec<(usize, W, I)>>,
}

impl<W: Copy + Add<Output = W> + Inf + Zero + Ord, I: Clone> WD<W, I> {
	pub fn new(n: usize) -> Self {
		Self {
			adj: vec![Vec::new(); n],
		}
	}
	pub fn len(&self) -> usize {
		self.adj.len()
	}
	pub fn dijkstra(&self, src: Vec<usize>) -> Vec<W> {
		//TODO: multi-source can be replaced by one mock node with 0 distance
		let mut dist = vec![W::inf(); self.len()];
		let mut pq = BinaryHeap::<(Reverse<W>, usize)>::new();
		for i in src {
			pq.push((Reverse(W::zero()), i));
			dist[i] = W::zero();
		}
		while pq.len() > 0 {
			let (distx, x) = pq.pop().unwrap();
			if distx.0 > dist[x] {
				continue;
			}
			for (y, xyw, _) in self.adj[x].iter() {
				if dist[*y] > dist[x] + *xyw {
					dist[*y] = dist[x] + *xyw;
					pq.push((Reverse(dist[*y]), *y));
				}
			}
		}
		dist
	}
	//return empty vector when negative cycle detected
	pub fn bellman_ford(&self, src: usize) -> Vec<W> {
		let n = self.len();
		let mut d = vec![W::inf(); n];
		d[src] = W::zero();
		for _epoch in 0..=n {
			let mut fin = true;
			for i in 0..n {
				for (j, w, _) in self.adj[i].iter() {
					if d[*j] > d[i] + *w {
						d[*j] = d[i] + *w;
						fin = false;
					}
				}
			}
			if fin {
				return d;
			}
		}
		vec![]
	}
	//return empty vector when negative cycle detected
	pub fn spfa(&self, src: usize) -> Vec<W> {
		let n = self.len();
		let mut d = vec![W::inf(); n];
		let mut p = Vec::new();
		let mut q = Vec::new();
		d[src] = W::zero();
		p.push(src);
		for _epoch in 0..n {
			for i in p.iter() {
				for (j, w, _) in self.adj[*i].iter() {
					if d[*j] > d[*i] + *w {
						d[*j] = d[*i] + *w;
						q.push(*j);
					}
				}
			}
			swap(&mut p, &mut q);
			q.clear();
			if p.is_empty() {
				return d;
			}
		}
		vec![]
	}
}

impl<W: Copy + Add<Output = W> + Inf + Zero + Ord> WD<W, ()> {
	pub fn add_edge(&mut self, from: usize, to: usize, w: W) {
		self.adj[from].push((to, w, ()));
	}
	pub fn reverse(&self) -> WD<W, ()> {
		let mut ret = WD::<W, ()>::new(self.len());
		for x in 0..self.len() {
			for (y, w, _) in self.adj[x].iter() {
				ret.add_edge(*y, x, *w);
			}
		}
		ret
	}
}
