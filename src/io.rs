pub struct BulkIO<'a>{
	it:SplitAsciiWhitespace<'a>,
	strout:String,
}
impl<'a> BulkIO<'a>{
	pub fn new()->BulkIO<'a>{
		static mut BUF:String = String::new();
		io::stdin().read_to_string(unsafe{&mut BUF}).unwrap();
		BulkIO{it:unsafe{BUF.split_ascii_whitespace()},strout:String::new()}
	}
	pub fn pop<T>(&mut self)->T where T:FromStr, T::Err:Debug{
		self.it.next().unwrap().parse().unwrap()
	}
	pub fn popn<T>(&mut self, n:usize)->impl Iterator<Item=T>+'a where T:FromStr,T::Err:Debug{
		let ret = self.it.clone().take(n).map(|x|x.parse().unwrap());
		for _ in 0..n{self.it.next();}
		ret
	}
	pub fn push<T>(&mut self, x:T)->&mut Self where T:ToString{
		self.strout.push_str(&x.to_string());
		self
	}
}
impl<'a> Drop for BulkIO<'a>{
	fn drop(&mut self) {
		print!("{}",self.strout);
		self.strout.clear();
	}
}

pub struct InteractIO<'a>{
	it:SplitAsciiWhitespace<'static>,
	stdin:StdinLock<'a>,
	stdout:StdoutLock<'a>,
}
static mut BUF:String = String::new();
impl<'a> InteractIO<'a>{
	pub fn new()->InteractIO<'a>{
		let mut stdin=io::stdin().lock();
		stdin.read_line(unsafe{&mut BUF}).unwrap();
		InteractIO{
			it:unsafe{BUF.split_ascii_whitespace()},
			stdin:stdin,
			stdout:io::stdout().lock()
		}
	}
	pub fn pop<T>(&mut self)->T where T:FromStr, T::Err:Debug{
		match self.it.next(){
			Some(x) => x.parse().unwrap(),
			None => {
				unsafe{BUF.clear()};
				self.stdin.read_line(unsafe{&mut BUF}).unwrap();
				self.it=unsafe{BUF.split_ascii_whitespace()};
				self.pop()
			}
		}
	}
	pub fn push<T>(&mut self, x:T)->&mut Self where T:ToString+Display{
		write!(self.stdout,"{}",x).unwrap(); self
	}
	pub fn flush(&mut self)->&mut Self{
		self.stdout.flush().unwrap(); self
	}
}

use std::fmt::{Debug, Display};
use std::io::{self,Read,BufRead,Write,StdoutLock,StdinLock};
use std::str::{FromStr,SplitAsciiWhitespace};
