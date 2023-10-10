
pub struct Tree{
	pub n:usize,
	pub g:Vec<Vec<usize>>,
}

impl Tree{
	pub fn new(n:usize)->Tree{
		Tree{n,g:vec![Vec::new();n]}
	}
	pub fn add_edge(&mut self, x:usize, y:usize){
		self.g[x].push(y);
		self.g[y].push(x);
	}
}

pub struct TreeRooted{
	pub n:usize,
	pub root:usize,
	pub p:Vec<usize>,
	pub ch:Vec<Vec<usize>>,
}

impl TreeRooted{
	pub fn new(tr:Tree, root:usize)->TreeRooted{
		let mut ret = TreeRooted { n:tr.n, root:root, p:vec![0;tr.n], ch:vec![Vec::new();tr.n] };
		ret.init_dfs(&tr, root, root);
		ret
	}
	pub fn init_dfs(&mut self, tr:&Tree, x:usize, px:usize){
		for y in tr.g[x].iter(){
			if *y!=px{
				self.p[*y]=x;
				self.ch[x].push(*y);
				self.init_dfs(tr, *y, x);
			}
		}
	}
}