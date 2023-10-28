use std::{cmp::Reverse, collections::BinaryHeap};

pub struct WD<W, I> {
	//W: 거리타입, I: 엣지 부가정보
	pub adj: Vec<Vec<(usize, W, I)>>,
}
impl<W, I> WD<W, I> {
	pub fn new(n: usize) -> Self {
		Self {
			adj: vec![Vec::new(); n],
		}
	}
	pub fn len(&self) -> usize {
		self.adj.len()
	}
	pub fn add_edge(&mut self, from: usize, to: usize, w: W, info: I) {
		self.adj[from].push((to, w, info));
	}
	pub fn dijkstra(&self, src: Vec<usize>) -> Vec<W> {
		let mut dist = vec![usize::MAX / 2; self.len()];
		let mut pq = BinaryHeap::<(Reverse<usize>, usize)>::new();
		for i in src {
			pq.push((Reverse(0), i));
			dist[i] = 0;
		}
		while pq.len() > 0 {
			let (distx, x) = pq.pop().unwrap();
			if distx.0 > dist[x] {
				continue;
			}
			for (y, xyw) in self.adj[x].iter() {
				if dist[*y] > dist[x] + *xyw {
					dist[*y] = dist[x] + *xyw;
					pq.push((Reverse(dist[*y]), *y));
				}
			}
		}
		dist
	}
	pub fn reverse(&self) -> WD<W, I> {
		let mut ret = WD::new(self.len());
		for x in 0..self.len() {
			for (y, w, i) in self.adj[x].iter() {
				ret.add_edge(*y, x, *w, *i);
			}
		}
		ret
	}
}
