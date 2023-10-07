pub fn input1<T>()->T where T:FromStr, <T as FromStr>::Err: fmt::Debug{
	let mut s = String::new();
	io::stdin().read_line(&mut s).unwrap();
	s.trim().parse().unwrap()
}

pub fn inputv<T>(s:&mut String)->Vec<T> where T:FromStr{
	io::stdin().read_line(s).unwrap();
	s.split_whitespace().flat_map(str::parse::<T>).collect()
}

pub fn inputi<'a,T>(s:&'a mut String)->impl Iterator<Item=T>+'a where T:FromStr+'a{
	io::stdin().read_line(s).unwrap();
	s.split_whitespace().flat_map(str::parse::<T>)
}

use std::fmt;
use std::io;
use std::str::FromStr;