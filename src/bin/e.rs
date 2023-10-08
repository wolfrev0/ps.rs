fn solve(){
}

fn main(){
	// let t:i32=input1();
	// for _ in 1..t+1
	// {print!("Case #{}: ",i);solve()}
	{solve();}
}

#[macro_export]
macro_rules! inputt {
	($($t: ty),+) => ({
		use std::io;
		let mut __a = String::new();
		io::stdin().read_line(&mut __a).unwrap();
		let mut __it = __a.split_whitespace();
		($(__it.next().unwrap().parse::<$t>().unwrap(),)+)
	})
}