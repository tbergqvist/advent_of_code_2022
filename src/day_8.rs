fn is_visible(forest: &Vec<Vec<u32>>, current_tree: u32, pos_y: usize, pos_x: usize) -> bool {
	if pos_x == 0 || pos_x == forest.len() - 1 ||pos_y == 0 || pos_y == forest.len() - 1 {
		return true;
	}
	forest[pos_y].iter().skip(pos_x + 1).all(|tree| *tree < current_tree) || 
	forest[pos_y].iter().take(pos_x).all(|tree| *tree < current_tree) ||
	forest.iter().skip(pos_y + 1).all(|row| row[pos_x] < current_tree) ||
	forest.iter().take(pos_y).all(|row| row[pos_x] < current_tree)
}

pub fn a(input: &str) -> u32 {
	let forest: Vec<Vec<u32>> = input.lines().map(|line| 
		line.chars()
			.map(|c| c.to_digit(10).unwrap())
			.collect()
	).collect();

	let mut visible_trees = 0;
	for y in 0..forest.len() {
		for x in 0..forest.len() {
			let cell = forest[y][x];
			if is_visible(&forest, cell, y, x) {
				visible_trees += 1;
			}
		}
	}
	visible_trees
}

fn score(forest: &Vec<Vec<u32>>, current_tree: u32, pos_y: usize, pos_x: usize) -> usize {
	if pos_x == 0 || pos_x == forest.len() - 1 ||pos_y == 0 || pos_y == forest.len() - 1 {
		return 0;
	}

	let right: Vec<(usize, &u32)> = forest[pos_y].iter().enumerate().skip(pos_x + 1).collect();
	let left: Vec<(usize, &u32)> = forest[pos_y].iter().enumerate().take(pos_x).collect();
	let down: Vec<(usize, u32)> = forest.iter().enumerate().skip(pos_y + 1).map(|(i, row)| (i, row[pos_x])).collect();
	let up: Vec<(usize, u32)> = forest.iter().enumerate().take(pos_y).map(|(i, row)| (i, row[pos_x])).collect();

	(right.into_iter().take_while(|(i, tree)| **tree < current_tree && *i != 0 && *i != forest.len() - 1).count() + 1) *
	(left.into_iter().rev().take_while(|(i, tree)| **tree < current_tree && *i != 0 && *i != forest.len() - 1).count() + 1) *
	(down.into_iter().take_while(|(i, tree)| *tree < current_tree && *i != 0 && *i != forest.len() - 1).count() + 1) *
	(up.into_iter().rev().take_while(|(i, tree)| *tree < current_tree && *i != 0 && *i != forest.len() - 1).count() + 1)
}

pub fn b(input: &str) -> usize {
	let forest: Vec<Vec<u32>> = input.lines().map(|line| 
		line.chars()
			.map(|c| c.to_digit(10).unwrap())
			.collect()
	).collect();

	let mut max_score = 0;
	for y in 0..forest.len() {
		for x in 0..forest.len() {
			let cell = forest[y][x];
			
			let score = score(&forest, cell, y, x);
			if score > max_score {
				max_score = score;
			}
		}
	}
	max_score
}