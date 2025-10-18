#![allow(static_mut_refs)]

fn main() {
	let mut io = psrs::io::BulkIO::new();
	let t: usize = io.pop();
	for i in 1..=t {
		io.push("Case #").push(i).push(": ");
		Solver::default().solve(&mut io);
	}
}

#[derive(Default)]
struct Solver {}
impl Solver {
	pub fn solve(&mut self, io: &mut psrs::io::BulkIO) {
		let n: usize = io.pop();
		io.push(n).push_endl();
	}
}
