pub fn a(input: &str) -> usize {
  input.lines()
	.filter(|line| {
		let values: Vec<i32> = line.split(',').flat_map(|val| val.split('-')).map(|s| s.parse().unwrap()).collect();
		(values[0] <= values[2] && values[1] >= values[3]) || (values[2] <= values[0] && values[3] >= values[1])
	}).count()
}

pub fn b(input: &str) -> usize {
  input.lines()
	.filter(|line| {
		let values: Vec<i32> = line.split(',').flat_map(|val| val.split('-')).map(|s| s.parse().unwrap()).collect();
		(values[1] >= values[2] && values[0] <= values[3]) || (values[2] >= values[1] && values[3] <= values[0])
	}).count()
}