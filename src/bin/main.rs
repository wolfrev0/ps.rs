use psrs::io::*;
use psrs::consts::*;

fn solve(){
}

fn main(){
	// let t:i32=read1();
	// for i in 1..t+1
	// {print!("Case #{}: ",i);solve()}
	{solve();}
}

#[macro_export]
macro_rules! readt {
	($($t: ty),+) => ({
		let mut __a = String::new();
		io::stdin().read_line(&mut __a).unwrap();
		let mut __it = __a.split_whitespace();
		($(__it.next().unwrap().parse::<$t>().unwrap(),)+)
	})
}
use std::{io, cmp::{min, max}};