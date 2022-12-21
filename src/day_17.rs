use std::collections::VecDeque;


type Block = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Position {
	x: i64,
	y: i64,
}

impl Position {
	fn move_down(&self) -> Position {
		Position {x: self.x, y: self.y - 1}
	}

fn move_wind(&self, wind_sign: u8) -> Position {
	let new_pos_diff = if wind_sign == b'>' {1} else {-1};
	let new_pos = self.x as i64 + new_pos_diff;

	Position {
		x: new_pos,
		y: self.y
	}
}
}

fn get_level_iter<'a>(pos: &'a Position, block: &'a Block) -> impl Iterator<Item = (i64, i64)> + 'a {
	(0..block.len()).into_iter().flat_map(move |local_y| 
		(0..block[local_y].len()).into_iter()
			.filter(move |local_x| block[local_y][*local_x] != '.')
			.map(move |local_x| (local_x as i64 + pos.x, local_y as i64 + pos.y)
		)
	)
}

fn collide(level: &Vec<Vec<char>>, pos: &Position, block: &Block) -> bool {
	for (global_x, global_y) in get_level_iter(&pos, block) {
		if global_y < 0 || global_x < 0 || global_y >= level.len() as i64 || global_x >= level[global_y as usize].len() as i64 {
			return true;
		}

		if level[global_y as usize][global_x as usize] == '#' {
			return true;
		}
	}
	false
}


fn fill_level(level: &mut Vec<Vec<char>>, pos: &Position, block: &Block) {
	for (global_x, global_y) in get_level_iter(&pos, block) {
		level[global_y as usize][global_x as usize] = '#';
	}
}

pub fn a(input: &str) -> usize {
	let blocks: Vec<Block> = vec![
		vec![
			vec!['#', '#', '#', '#']
		],
		vec![
			vec!['.', '#', '.'],
			vec!['#', '#', '#'],
			vec!['.', '#', '.'],
		],
		vec![
			vec!['#', '#', '#'],
			vec!['.', '.', '#'],
			vec!['.', '.', '#'],
		],
		vec![
			vec!['#'],
			vec!['#'],
			vec!['#'],
			vec!['#'],
		],
		vec![
			vec!['#', '#'],
			vec!['#', '#'],
		]
	];

	let mut level = vec![vec!['.';7];4];

	let mut wind_iter = input.trim().bytes().cycle();

	for block in blocks.iter().cycle().take(2022) {
		let highest_point = level.iter().enumerate().rev().find(|(_, row)| row.iter().any(|c| *c == '#')).map(|(i, _)| i + 1).unwrap_or(0);

		let correct_size = highest_point + 3 + block.len();
		while level.len() > correct_size {
			level.pop();
		}

		while level.len() < correct_size {
			level.push(vec!['.'; 7]);
		}

		let mut block_position: Position = Position { x: 2, y: level.len() as i64 - block.len() as i64 };

		loop {
			let wind_pos = block_position.move_wind(wind_iter.next().unwrap());
			if !collide(&level, &wind_pos, block) {
				block_position = wind_pos;
			}

			let down_pos = block_position.move_down();
			if !collide(&level, &down_pos, block) {
				block_position = down_pos;
			} else {
				fill_level(&mut level, &block_position, block);
				break;
			}
		}
	};

	level.iter().enumerate().rev().find(|(_, row)| row.iter().any(|c| *c == '#')).map(|(i, _)| i + 1).unwrap()
}

fn draw_grid(level: &Vec<Vec<char>>) {
	for y in (0..level.len()).rev() {
		for x in 0..level[y].len() {
			let cell = level[y][x];
			print!("{}", cell);
		}	
		println!();
	}
}

pub fn b(input: &str) -> i64 {
	0
}


