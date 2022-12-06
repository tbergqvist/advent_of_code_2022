pub fn a(input: &str) -> String {
	let mut stacks = vec![vec![]; 9];
	input.lines().take(8).for_each(|line| {
		let mut push = |index: usize| {
			let char = line.chars().nth(index * 4 + 1).unwrap();
			if char != ' ' {
				stacks[index].push(char);
			}
		};

		for i in 0..9 {
			push(i);
		}
	});

	for v in &mut stacks {
			v.reverse();
	}

  input.lines()
		.skip(10)
		.map(|line| {
			let values: Vec<usize> = line.split(' ').filter_map(|val| val.parse().ok()).collect();
			(values[0], values[1], values[2])
		})
		.for_each(|(amount, from, to)| {
			for _ in 0..amount {
				let from_vec = &mut stacks[from - 1];
				let val = from_vec.pop();
				let to_vec = &mut stacks[to - 1];

					to_vec.push(val.unwrap());
			}
		});

	stacks
		.iter()
		.map(|v| v.last().unwrap())
		.collect()
}

pub fn b(input: &str) -> String {
	let mut stacks = vec![vec![]; 9];
	input
		.lines()
		.take(8)
		.for_each(|line| {
			let mut push = |index: usize| {
				let char = line.chars().nth(index * 4 + 1).unwrap();
				if char != ' ' {
					stacks[index].push(char);
				}
			};

			for i in 0..9 {
				push(i);
			}
	});

	for v in &mut stacks {
		v.reverse();
	}

  input.lines()
		.skip(10)
		.map(|line| {
			let values: Vec<usize> = line.split(' ').filter_map(|val| val.parse().ok()).collect();
			(values[0], values[1], values[2])
		})
		.for_each(|(amount, from, to)| {
				let mut values: Vec<char> = Vec::new();
				for _ in 0..amount {
					let from_vec = &mut stacks[from - 1];
					let val = from_vec.pop();
	
					values.push(val.unwrap());
				}
				values.iter().rev().for_each(|v| {
					let to_vec = &mut stacks[to - 1];
					to_vec.push(*v);
				});
		});

	stacks
		.iter()
		.map(|v| v.last().unwrap())
		.collect()
}