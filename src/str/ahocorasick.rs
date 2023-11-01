#[derive(Clone)]
pub struct Node<const M: usize> {
	p: usize, //parent_link
	c: u8,    //char from parent to here
	s: usize, //suffix_link
	o: usize, //output_link
	succ: [usize; M],
}
impl<const M: usize> Node<M> {
	pub fn new(p: usize, c: u8) -> Self {
		Self {
			p,
			c,
			s: usize::MAX,
			o: usize::MAX,
			succ: [usize::MAX; M],
		}
	}
}

pub struct AhoCorasick<const M: usize> {
	a: Vec<Node<M>>,
}
impl<const M: usize> AhoCorasick<M> {
	pub fn new() -> Self {
		Self {
			a: vec![Node::new(0, u8::MAX); 1],
		}
	}
	pub fn alloc(&mut self, x: Node<M>) -> usize {
		self.a.push(x);
		self.a.len() - 1
	}
	pub fn get_succ(&mut self, id: usize, ch: u8) -> &mut usize {
		&mut self.a[id].succ[ch as usize]
	}
	pub fn add(&mut self, id: usize, mut it: impl Iterator<Item = u8>) {
		match it.next() {
			None => self.a[id].o = id,
			Some(ch) => {
				if *self.get_succ(id, ch) == usize::MAX {
					*self.get_succ(id, ch) = self.alloc(Node::new(id, ch))
				}
				let tmp = *self.get_succ(id, ch);
				self.add(tmp, it);
			}
		}
	}
	pub fn transit(&mut self, id: usize, ch: u8) -> usize {
		if *self.get_succ(id, ch) != usize::MAX {
			*self.get_succ(id, ch)
		} else if self.a[id].p == id {
			id
		} else {
			let tmp = self.get_slink(id);
			self.transit(tmp, ch)
		}
	}
	pub fn get_slink(&mut self, id: usize) -> usize {
		if self.a[id].p == self.a[self.a[id].p].p {
			self.a[id].p
		} else {
			if self.a[id].s == usize::MAX {
				let tmp = self.get_slink(self.a[id].p);
				self.a[id].s = self.transit(tmp, self.a[id].c)
			}
			self.a[id].s
		}
	}
	pub fn get_olink(&mut self, id: usize) -> usize {
		if self.a[id].p == id {
			id
		} else {
			if self.a[id].o == usize::MAX {
				let tmp = self.get_slink(id);
				self.a[id].o = self.get_olink(tmp)
			}
			self.a[id].o
		}
	}
}
