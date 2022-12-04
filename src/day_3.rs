pub fn a(input: &str) -> i32 {
  input.lines()
  .map(|i| i.split_at(i.len() / 2))
	.filter_map(|(comp1, comp2)| comp1.chars().find(|item| comp2.contains(*item)))
	.map(|c| if c.is_uppercase() {
		c as i32 - 38
	} else {
		c as i32 - 96
	})
	.sum()
}

pub fn b(input: &str) -> i32 {
  let lines: Vec<&str> = input.lines().collect();
	lines.chunks(3)
		.filter_map(|chunks| {
			let [a, b, c] = chunks else { panic!() };
			a.chars().find(|item| b.contains(*item) && c.contains(*item))
		})
		.map(|c| if c.is_uppercase() {
			c as i32 - 38
		} else {
			c as i32 - 96
		}).sum()
		
}