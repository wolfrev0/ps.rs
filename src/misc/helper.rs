use std::collections::BTreeMap;

#[macro_export]
macro_rules! negi {
	($vec:expr, $idx:expr) => {
		$vec[if $idx < 0 {
			$idx as i32 + $vec.len() as i32
		} else {
			$idx as i32
		} as usize]
	};
}
pub static DIR4: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub static DIR8: [(i32, i32); 8] = [
	(0, 1),
	(-1, 1),
	(-1, 0),
	(-1, -1),
	(0, -1),
	(1, -1),
	(1, 0),
	(1, 1),
];
pub static ENDL: char = '\n';

pub fn classify<T>(a: &Vec<T>) -> BTreeMap<T, Vec<usize>>
where
	T: Clone + Ord,
{
	let mut ret = BTreeMap::new();
	for i in 0..a.len() {
		if ret.contains_key(&a[i]) == false {
			ret.insert(a[i].clone(), Vec::new());
		}
		ret.get_mut(&a[i]).unwrap().push(i)
	}
	ret
}

pub fn permu_inv<T>(a: &Vec<T>) -> Vec<T>
where
	T: Copy,
	usize: From<T>,
	T: From<usize>,
{
	let mut b = vec![T::from(0); a.len()];
	for i in 0..a.len() {
		b[usize::from(a[i])] = T::from(i);
	}
	b
}

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;
	#[test]
	fn test_negi_0() {
		assert_eq!(negi!(vec![3, 4, 5, 6], -2 as i64), 5);
		assert_eq!(negi!(negi!(vec![vec![3, 4]; 2], -1 as i64), -2 as i32), 3);
		assert_eq!(negi!("asdf".as_bytes(), -3), 's' as u8);
		assert_eq!(negi!(VecDeque::from_iter([1, 2, 3]), -1), 3)
	}
}
