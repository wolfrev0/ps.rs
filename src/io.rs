static mut BUF: String = String::new();

pub struct BulkIO {
	it: SplitAsciiWhitespace<'static>,
	strout: String,
}
impl BulkIO {
	pub fn new() -> BulkIO {
		io::stdin().read_to_string(unsafe { &mut BUF }).unwrap();
		BulkIO {
			it: unsafe { BUF.split_ascii_whitespace() },
			strout: String::new(),
		}
	}
	pub fn pop<T>(&mut self) -> T
	where
		T: FromStr,
		T::Err: Debug,
	{
		self.it.next().unwrap().parse().unwrap()
	}
	pub fn popn<T>(&mut self, n: usize) -> impl Iterator<Item = T>
	where
		T: FromStr,
		T::Err: Debug,
	{
		let ret = self.it.clone().take(n).map(|x| x.parse().unwrap());
		for _ in 0..n {
			self.it.next();
		}
		ret
	}
	pub fn popstr(&mut self) -> &str {
		self.it.next().unwrap()
	}
	pub fn push<T>(&mut self, x: T) -> &mut Self
	where
		T: ToString,
	{
		self.strout.push_str(&x.to_string());
		self
	}

	//helper functions
	pub fn popc(&mut self) -> u8 {
		self.popstr().bytes().next().unwrap()
	}
	pub fn popvu(&mut self, n: usize) -> Vec<usize> {
		Vec::from_iter(self.popn::<usize>(n))
	}
	pub fn popvi(&mut self, n: usize) -> Vec<isize> {
		Vec::from_iter(self.popn::<isize>(n))
	}
	pub fn push_space(&mut self) -> &mut Self {
		self.push(' ')
	}
	pub fn push_endl(&mut self) -> &mut Self {
		self.push('\n')
	}
}
impl Drop for BulkIO {
	fn drop(&mut self) {
		print!("{}", self.strout);
		self.strout.clear();
	}
}

pub struct InteractIO {
	it: SplitAsciiWhitespace<'static>,
	stdin: StdinLock<'static>,
	stdout: StdoutLock<'static>,
}
impl InteractIO {
	pub fn new() -> InteractIO {
		let mut stdin = io::stdin().lock();
		stdin.read_line(unsafe { &mut BUF }).unwrap();
		InteractIO {
			it: unsafe { BUF.split_ascii_whitespace() },
			stdin: stdin,
			stdout: io::stdout().lock(),
		}
	}
	pub fn pop<T>(&mut self) -> T
	where
		T: FromStr,
		T::Err: Debug,
	{
		match self.it.next() {
			Some(x) => x.parse().unwrap(),
			None => {
				unsafe { BUF.clear() };
				self.stdin.read_line(unsafe { &mut BUF }).unwrap();
				self.it = unsafe { BUF.split_ascii_whitespace() };
				self.pop()
			}
		}
	}
	pub fn push<T>(&mut self, x: T) -> &mut Self
	where
		T: ToString + Display,
	{
		write!(self.stdout, "{}", x).unwrap();
		self
	}
	pub fn flush(&mut self) -> &mut Self {
		self.stdout.flush().unwrap();
		self
	}
}

use std::fmt::{Debug, Display};
use std::io::{self, BufRead, Read, StdinLock, StdoutLock, Write};
use std::str::{FromStr, SplitAsciiWhitespace};
