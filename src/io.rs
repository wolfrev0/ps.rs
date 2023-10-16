pub struct BulkIO{
	it:SplitAsciiWhitespace<'static>,
	strout:String,
}
impl BulkIO{
	pub fn new()->BulkIO{
		static mut BUF:String = String::new();
		io::stdin().read_to_string(unsafe{&mut BUF}).unwrap();
		BulkIO{it:unsafe{BUF.split_ascii_whitespace()},strout:String::new()}
	}
	pub fn pop<T>(&mut self)->T where T:FromStr, T::Err:Debug{
		self.it.next().unwrap().parse().unwrap()
	}
	pub fn push<T>(&mut self, x:T)->&mut Self where T:ToString{
		self.strout.push_str(&x.to_string());
		self
	}
}
impl Drop for BulkIO{
	fn drop(&mut self) {
		print!("{}",self.strout);
		self.strout.clear();
	}
}

pub struct InteractIO<'a>{
	it:SplitAsciiWhitespace<'static>,
	stdout:StdoutLock<'a>,
}
static mut BUF:String = String::new();
impl<'a> InteractIO<'a>{
	pub fn new()->InteractIO<'a>{
		unsafe{BUF = String::new()};
		io::stdin().read_line(unsafe{&mut BUF}).unwrap();
		InteractIO{it:unsafe{BUF.split_ascii_whitespace()},stdout:io::stdout().lock()}
	}
	pub fn pop<T>(&mut self)->T where T:FromStr, T::Err:Debug{
		match self.it.next(){
			Some(x) => x.parse().unwrap(),
			None => {
				unsafe{BUF = String::new()};
				io::stdin().read_line(unsafe{&mut BUF}).unwrap();
				self.it=unsafe{BUF.split_ascii_whitespace()};
				self.pop()
			}
		}
	}
	pub fn push<T>(&mut self, x:T)->&mut Self where T:ToString+Display{
		write!(self.stdout,"{}",x).unwrap();
		self
	}
	pub fn flush(&mut self)->&mut Self{
		self.stdout.flush().unwrap(); self
	}
}

use std::fmt::{Debug, Display};
use std::io::{self,Read,Write,StdoutLock};
use std::str::{FromStr,SplitAsciiWhitespace};
