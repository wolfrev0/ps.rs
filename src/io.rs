pub fn input1<T>()->T where T:FromStr, <T as FromStr>::Err: fmt::Debug{
	let mut s = String::new();
	io::stdin().read_line(&mut s).unwrap();
	s.trim().parse().unwrap()
}

pub fn inputv<T>()->Vec<T> where T:FromStr{
	inputi(&mut String::new()).collect()
}

pub fn inputi<'a,T>(s:&'a mut String)->impl Iterator<Item=T>+'a where T:FromStr+'a{
	io::stdin().read_line(s).unwrap();
	s.split_whitespace().flat_map(str::parse::<T>)
}

pub struct Printer{
	s:String
}
impl Printer{
	pub fn new()->Printer{
		Printer{s:String::new()}
	}
	pub fn push<T>(&mut self, x:T, delim:&str)->&mut Self where T:ToString{
		self.s.push_str(&x.to_string());
		self.s.push_str(delim);
		self
	}
	pub fn flush(&mut self)->&mut Self{
		println!("{}",self.s);
		self.s.clear();
		self
	}
}
impl Drop for Printer{
	fn drop(&mut self) {
		println!("{}",self.s);
	}
}

use std::fmt;
use std::io;
use std::str::FromStr;
