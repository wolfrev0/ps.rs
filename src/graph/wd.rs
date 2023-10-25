use std::{collections::BinaryHeap, cmp::Reverse};

pub struct WD{
	pub adj:Vec<Vec<(usize,usize)>>
}
impl WD{
	pub fn new(n:usize)->WD{
		WD{adj:vec![Vec::new();n]}
	}
	pub fn len(&self)->usize{self.adj.len()}
	pub fn add_edge(&mut self, from:usize, to:usize, weight:usize){
		self.adj[from].push((to,weight));
	}
	pub fn dijkstra(&self, src:Vec<usize>)->Vec<usize>{
		let mut dist=vec![usize::MAX/2;self.len()];
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
	pub fn reverse(&self)->WD{
		let mut ret = WD::new(self.len());
		for x in 0..self.len(){
			for (y,w) in self.adj[x].iter(){
				ret.add_edge(*y, x, *w);
			}
		}
		ret
	}
}

