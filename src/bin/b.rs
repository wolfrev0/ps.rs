#![allow(non_upper_case_globals, static_mut_refs)]

fn main() {
	let mut io = psrs::io::BulkIO::new();
	let t: usize = 1; //io.pop();
	for _i in 1..=t {
		// io.push("Case #").push(_i).push(": ");
		Solver::new(&mut io).solve();
	}
}

#[derive(Default)]
struct Solver {
	n: usize,
}
impl Solver {
	pub fn new(io: &mut psrs::io::BulkIO) -> Self {
		let n = io.pop();
		Self {
			n,
			..Default::default()
		}
	}
	pub fn solve(&mut self) {
		println!("{}", self.n);
	}
}
