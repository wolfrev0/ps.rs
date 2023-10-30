use std::{cell::Cell, ops::Add};

use crate::math::structs::{Inf, Zero};

use super::{ud::UD, wd::WD};

#[derive(Clone)]
pub struct FlowInfo {
	pub resi: usize,      //residual index
	pub cap: Cell<usize>, //capacity
}

impl UD<FlowInfo> {
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
	pub fn max_flow(&mut self, src: usize, snk: usize, flow_bound: usize) -> usize {
		fn dfs(
			adj: &Vec<Vec<(usize, FlowInfo)>>,
			snk: usize,
			x: usize,
			flow: usize,
			vis: &mut Vec<bool>,
		) -> usize {
			if x == snk {
				return flow;
			}
			vis[x] = true;
			for (y, FlowInfo { resi, cap }) in adj[x].iter() {
				if !vis[*y] && cap.get() > 0 {
					let flowy = dfs(adj, snk, *y, flow.min(cap.get()), vis);
					if flowy > 0 {
						cap.set(cap.get() - flowy);
						let cap_resi = &adj[*y][*resi].1.cap;
						cap_resi.set(cap_resi.get() + flowy);
						return flowy;
					}
				}
			}
			0
		}
		let mut ret = 0;
		loop {
			let flow = dfs(
				&self.adj,
				snk,
				src,
				flow_bound,
				&mut vec![false; self.len()],
			);
			if flow == 0 {
				break;
			}
			ret += flow;
		}
		ret
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
