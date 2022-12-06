pub fn a(input: &str) -> String {
	let mut bla = vec![
		vec!['T', 'F', 'V', 'Z', 'C', 'W', 'S', 'Q'],
		vec!['B', 'R', 'Q'],
		vec!['S', 'M', 'P', 'Q', 'T', 'Z', 'B'],
		vec!['H', 'Q', 'R', 'F', 'V', 'D'],
		vec!['P', 'T', 'S', 'B', 'D', 'L', 'G', 'J'],
		vec!['Z', 'T', 'R', 'W'],
		vec!['J', 'R', 'F', 'S', 'N', 'M', 'Q', 'H'],
		vec!['W', 'H', 'F', 'N', 'R'],
		vec!['B', 'R', 'P', 'Q', 'T', 'Z', 'J'],
	];

	for v in &mut bla {
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
				let from_vec = &mut bla[from - 1];
				let val = from_vec.pop();
				let to_vec = &mut bla[to - 1];

					to_vec.push(val.unwrap());
			}
		});

	bla
		.iter()
		.map(|v| v.last().unwrap())
		.collect()
}

pub fn b(input: &str) -> String {
	let mut bla = vec![
		vec!['T', 'F', 'V', 'Z', 'C', 'W', 'S', 'Q'],
		vec!['B', 'R', 'Q'],
		vec!['S', 'M', 'P', 'Q', 'T', 'Z', 'B'],
		vec!['H', 'Q', 'R', 'F', 'V', 'D'],
		vec!['P', 'T', 'S', 'B', 'D', 'L', 'G', 'J'],
		vec!['Z', 'T', 'R', 'W'],
		vec!['J', 'R', 'F', 'S', 'N', 'M', 'Q', 'H'],
		vec!['W', 'H', 'F', 'N', 'R'],
		vec!['B', 'R', 'P', 'Q', 'T', 'Z', 'J'],
	];

	for v in &mut bla {
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
					let from_vec = &mut bla[from - 1];
					let val = from_vec.pop();
	
					values.push(val.unwrap());
				}
				values.iter().rev().for_each(|v| {
					let to_vec = &mut bla[to - 1];
					to_vec.push(*v);
				});
		});

	bla
		.iter()
		.map(|v| v.last().unwrap())
		.collect()
}