fn find_at_cycle(input: &str, cycle: usize) -> i32 {
	input.lines()
	.flat_map(|line| {
		line.split(' ').map(|l| l.parse::<i32>().ok())
	})
	.take(cycle)
	.flatten()
	.sum::<i32>() + 1
}

pub fn a(input: &str) -> i32 {
	[20, 60, 100, 140, 180, 220]
		.into_iter()
		.map(|cycles| find_at_cycle(input, cycles - 1) * cycles as i32)
	.sum()
}

pub fn b(input: &str) -> String {
	let mut board = vec![vec!['.';40];6];
	
	for i in 0..240 {
		let val = find_at_cycle(input, i);
		if (val - (i % 40) as i32).abs() <= 1 {
			let y = i / 40;
			let x = i % 40;
			board[y][x] = '#';
		}
	};
	
	board.into_iter()
		.fold(String::new(), |s, row| {
			s + "\n" + &row.into_iter().collect::<String>()
		})
}