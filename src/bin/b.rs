#![allow(non_upper_case_globals, static_mut_refs)]

fn main() {
	let mut io = psrs::io::BulkIO::new();
	let t: usize = 1; //io.pop();
	for _i in 1..=t {
		// io.push("Case #").push(_i).push(": ");
		unsafe {
			solve(&mut io);
		}
	}
}

unsafe fn solve(io: &mut psrs::io::BulkIO) {
	let _ = io;
}
