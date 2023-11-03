//only finds odd palindromes
//to find even palindromes, convert string "abcd" to "#a#b#c#d#"
//변형후 manacher[i]은 변형전 팰린드롬 길이가 된다.
pub fn manacher<T: PartialEq>(a: Vec<T>) -> Vec<usize> {
	let mut b = vec![0; a.len()];
	let mut c = usize::MAX;
	for i in 0..a.len() {
		if c < usize::MAX && i <= c + b[c] {
			b[i] = (c + b[c] - i).min(b[2 * c - i] /*c - (i - c)*/);
		}
		while i + b[i] + 1 < a.len() && i >= b[i] + 1 && a[i + b[i] + 1] == a[i - b[i] - 1] {
			b[i] += 1;
		}
		if c == usize::MAX || c + b[c] < i + b[i] {
			c = i;
		}
	}
	b
}
