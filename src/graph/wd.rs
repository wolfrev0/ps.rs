use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

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
