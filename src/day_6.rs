use std::collections::HashSet;

fn find_marker_index(input: &str, num: usize) -> usize {
	input
		.as_bytes()
		.windows(num)
		.enumerate()
		.filter(|(_, chunk)| {
			let set: HashSet<&u8> = chunk.iter().collect();
			set.len() == num
		})
		.map(|(i, _)| i)
		.next()
		.unwrap() + num
}

pub fn a(input: &str) -> usize {
	find_marker_index(input, 4)
}

pub fn b(input: &str) -> usize {
	find_marker_index(input, 14)
}