use std::{collections::BinaryHeap, cmp::Reverse};

pub struct GraphWD{
	pub adj:Vec<Vec<(usize,usize)>>
}
impl GraphWD{
	pub fn new(n:usize)->GraphWD{
		GraphWD{adj:vec![Vec::new();n]}
	}
	pub fn size(&self)->usize{self.adj.len()}
	pub fn add_edge(&mut self, from:usize, to:usize, weight:usize){
		self.adj[from].push((to,weight));
	}
	pub fn dijkstra(&self, src:Vec<usize>)->Vec<usize>{
		let mut dist=vec![usize::MAX/2;self.size()];
		let mut pq = BinaryHeap::<(Reverse<usize>,usize)>::new();
		for i in src{
			pq.push((Reverse(0),i));
			dist[i]=0;
		}
		while pq.len()>0{
			let (distx,x) = pq.pop().unwrap();
			if distx.0 > dist[x]{
				continue;
			}
			for (y,xyw) in self.adj[x].iter(){
				if dist[*y]>dist[x]+*xyw{
					dist[*y]=dist[x]+*xyw;
					pq.push((Reverse(dist[*y]),*y));
				}
			}
		}
		dist
	}
	pub fn reverse(&self)->GraphWD{
		let mut ret = GraphWD::new(self.size());
		for x in 0..self.size(){
			for (y,w) in self.adj[x].iter(){
				ret.add_edge(*y, x, *w);
			}
		}
		ret
	}
}

