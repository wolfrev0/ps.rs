#[macro_export]
macro_rules! negi {
	($vec:expr, $idx:expr) => {
		$vec[if $idx<0 {$idx as i32 + $vec.len() as i32} else {$idx} as usize]
	};
}