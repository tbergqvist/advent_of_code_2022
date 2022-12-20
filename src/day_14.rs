#[derive(Clone, Copy, PartialEq)]
struct Position<T> {
	x: T,
	y: T,
}

type UPosition = Position<usize>;
type IPosition = Position<i32>;
type Level = Vec<Vec<char>>;

fn parse_level(input: &str) -> Level {
	let walls: Vec<Vec<IPosition>> = input.lines()
		.map(|line| 
			line.split(" -> ").map(|pair| {
				let mut iter = pair.split(",").map(|p| p.parse().unwrap());
				Position { x: iter.next().unwrap(), y: iter.next().unwrap() }
			}).collect()
	).collect();

	let mut level = vec![vec!['.'; 1000]; 501];
	for wall in walls {
		let mut current_pos = wall[0].clone();
		for line in wall {
			let diff_x = line.x - current_pos.x;
			let diff_y = line.y - current_pos.y;
			loop {
				level[current_pos.y as usize][current_pos.x as usize] = '#';
				if line.x == current_pos.x && line.y == current_pos.y {
					break;
				}
				current_pos = Position {
					x: current_pos.x + diff_x.signum(),
					y: current_pos.y + diff_y.signum(),
				};
			}
		}
	}
	level
}

fn next_position(level: &Level, pos: UPosition) -> Option<UPosition> {
	if level[pos.y + 1][pos.x] == '.' { Some(UPosition{ y: pos.y + 1, x: pos.x }) }
	else if level[pos.y + 1][pos.x - 1] == '.' { Some(UPosition{ y: pos.y + 1, x: pos.x - 1 }) }
	else if level[pos.y + 1][pos.x + 1] == '.' { Some(UPosition{ y: pos.y + 1, x: pos.x + 1 }) }
	else { None }
}

fn drop_sandcorn(level: &Level) -> Option<UPosition> {
	let mut sandcorn_position = Position { y: 0, x: 500 };

	loop {
		let new_position = next_position(level, sandcorn_position);
		if new_position.is_none() {
			return Some(sandcorn_position);
		}
		if new_position.unwrap().y >= 500 {
			return None;
		}

		sandcorn_position = new_position.unwrap();
	}
}

fn draw_grid(level: &Level) {
	for y in 0..200 {
		for x in 400..600 {
			let cell = level[y][x];
			print!("{}", cell);
		}	
		println!();
	}
}

pub fn a(input: &str) -> usize {
	let mut level = parse_level(input);
	let mut i = 0;
	while let Some(pos) = drop_sandcorn(&level) {
		level[pos.y][pos.x] = 'o';
		i += 1;
	}
	i
}

pub fn b(input: &str) -> usize {
	let mut level = parse_level(input);
	let max_height = level.iter().enumerate().filter_map(|(i, bla)| 
		if bla.iter().any(|c| c != &'.') {
			Some(i)
		} else {
			None
		}
	).max().unwrap() + 2;

	for x in 0..1000 {
		level[max_height][x] = '#';
	}

	let mut i = 0;
	while let Some(pos) = drop_sandcorn(&level) {
		level[pos.y][pos.x] = 'o';
		i += 1;
		if pos.y == 0 {
			break;
		}
	}

	i
}


