
pub struct Tree{
	pub g:Vec<Vec<usize>>,
}
impl Tree{
	pub fn new(n:usize)->Tree{
		Tree{g:vec![Vec::new();n]}
	}
	pub fn size(&self)->usize{ self.g.len() }
	pub fn add_edge(&mut self, x:usize, y:usize){
		self.g[x].push(y);
		self.g[y].push(x);
	}
}

pub struct TreeRooted{
	pub root:usize,
	pub p:Vec<usize>,
	pub ch:Vec<Vec<usize>>,
}
impl TreeRooted{
	pub fn new(tr:Tree, root:usize)->TreeRooted{
		let mut ret = TreeRooted { root:root, p:vec![0;tr.size()], ch:vec![Vec::new();tr.size()] };
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