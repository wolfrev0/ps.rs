pub struct Tree{
	pub g:Vec<Vec<usize>>,
}
impl Tree{
	pub fn new(n:usize)->Tree{
		Tree{g:vec![Vec::new();n]}
	}
	pub fn len(&self)->usize{ self.g.len() }
	pub fn add_edge(&mut self, x:usize, y:usize){
		self.g[x].push(y);
		self.g[y].push(x);
	}
	//TODO: Tree Compress(https://www.facebook.com/codingcompetitions/hacker-cup/2023/round-2/problems/C)
}

pub struct TreeRooted{
	pub root:usize,
	pub p:Vec<usize>,
	pub ch:Vec<Vec<usize>>,
	pub sz:Vec<usize>,
}
impl TreeRooted{
	pub fn len(&self)->usize{self.p.len()}
	pub fn new(tr:Tree, root:usize)->TreeRooted{
		fn init_dfs(mut s:TreeRooted, tr:&Tree, x:usize, px:usize)->TreeRooted{
			for y in tr.g[x].iter(){
				if *y!=px{
					s=init_dfs(s, tr, *y, x);
					s.p[*y]=x;
					s.ch[x].push(*y);
					s.sz[x]+=s.sz[*y];
				}
			}
			s.sz[x]+=1;
			// make largest child subtree to be first element.
			// It's useful When doing HLD.
			for i in 1..s.ch[x].len(){
				if s.sz[s.ch[x][0]] < s.sz[s.ch[x][i]]{
					s.ch[x].swap(0,i)
				}
			}
			s
		}
		init_dfs(
			TreeRooted{
				root:root,
				p:vec![0;tr.len()],
				ch:vec![Vec::new();tr.len()],
				sz:vec![0;tr.len()],
			}, &tr, root, root)
	}
}