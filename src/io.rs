pub fn read1<T>()->T where T:FromStr, <T as FromStr>::Err: fmt::Debug{
	let mut s = String::new();
	io::stdin().read_line(&mut s).unwrap();
	s.trim().parse().unwrap()
}

pub fn readv<T>()->Vec<T> where T:FromStr{
	let mut s = String::new();
	io::stdin().read_line(&mut s).unwrap();
	s.split_whitespace().flat_map(str::parse::<T>).collect()
}

use std::fmt;
use std::io;
use std::str::FromStr;