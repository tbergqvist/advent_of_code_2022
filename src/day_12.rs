use pathfinding::prelude::{astar};

#[derive(Debug, Clone, Copy)]
struct Pos {
	y: i64,
	x: i64,
}

fn find_letter_pos(grid: &Vec<Vec<u8>>, b: u8) -> Pos {
	grid.iter()
	.enumerate()
	.find_map(|(y, row)| row.iter()
		.enumerate()
		.find(|c| *c.1 == b)
		.map(|(x, _)| Pos {y: y as i64, x: x as i64})
	).unwrap()
}

fn find_path(grid: &Vec<Vec<u8>>, start: Pos, end: Pos) -> Option<usize> {
	let size_x: i64 = grid[0].len() as i64 - 1;
	let size_y: i64 = grid.len() as i64 - 1;
  astar(&(start.x, start.y), |&(x, y)| {
		let current = grid[y as usize][x as usize];
		let current = if current == b'E' {b'z'} else {current};
		let grid = &grid;
    vec![(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)]
      .into_iter()
      .filter(|&(x, y)| x >= 0 && y >= 0 && x <= size_x && y <= size_y)
			.filter(move |&(new_x, new_y)| {
				let new = grid[new_y as usize][new_x as usize];
				let new = if new == b'E' {b'z'} else {new};
				let bla = new as i32 - current as i32;

				current == b'S' || bla <= 1
			})
      .map(|(new_x, new_y)| {
        ((new_x, new_y), 1 as i64)
      })
    },
    |&(x, y)| {
      end.x - x + end.y - y
    }, |&(x, y)| {
      x == end.x && y == end.y
    }
  ).map(|bla| bla.0.len() - 1)
}

pub fn a(input: &str) -> usize {
	let grid: Vec<Vec<u8>> = input.lines()
		.map(|line| line.chars().map(|c| c as u8).collect())
		.collect();

	let start = find_letter_pos(&grid, b'S');
	let goal = find_letter_pos(&grid, b'E');

	find_path(&grid, start, goal).unwrap()
}

pub fn b(input: &str) -> usize {
	let grid: Vec<Vec<u8>> = input.lines()
		.map(|line| line.chars().map(|c| c as u8).collect())
		.collect();

	let starts = grid.iter()
		.enumerate()
		.flat_map(|(y, row)| row.iter()
			.enumerate()
			.filter(|c| *c.1 == b'a')
			.map(move |(x, _)| Pos {y: y as i64, x: x as i64})
		);

	let end = find_letter_pos(&grid, b'E');

	starts.into_iter().filter_map(|start| {
		find_path(&grid, start, end.clone())
	})
	.min()
	.unwrap()
}


