#[derive(Debug, Clone, PartialEq)]
enum Item {
	Value(usize),
	List(Vec<Item>),
}

pub fn a(input: &str) -> usize {
	let pairs: Vec<Vec<Item>> = input.split("\n\n").flat_map(|group| 
		group.lines()
			.map(|line| {
				let mut iter = line.chars();
				parse_array(&mut iter)
			})
	).collect();
	pairs.chunks(2).enumerate().filter(|(_, chunks)| {
		let [left, right] = chunks else { panic!() };
		compare(left, right).unwrap()
	})
	.map(|b| b.0 + 1)
	.sum()
}

fn compare(left: &Vec<Item>, right: &Vec<Item>) -> Option<bool> {
	let min = std::cmp::min(left.len(), right.len());
		for i in 0..min {
			let item1 = &left[i];
			let item2 = &right[i];
			let diff = match (item1, item2) {
					(Item::Value(v1), Item::Value(v2)) => if v1 == v2 {None} else { Some(v1 < v2) },
					(Item::List(v1), Item::List(v2)) => compare(v1, v2),
					(Item::List(v1), Item::Value(v2)) => compare(v1, &vec![Item::Value(*v2)]),
					(Item::Value(v1), Item::List(v2)) => compare(&vec![Item::Value(*v1)], v2),
			};

			if diff.is_some() {
				return diff;
			}
		}

		if left.len() == right.len() {
			None
		} else {
			Some(left.len() < right.len())
		}
}

fn parse_array(iter: &mut std::str::Chars) -> Vec<Item> {
	let mut current_list: Vec<Item> = Vec::new();
	while let Some(current) = iter.next() {
		if current == ',' {
		} else if current == '[' {
			let list = parse_array(iter);
			current_list.push(Item::List(list));
		} else if current == ']' {
			return current_list;
		} else {
			let mut num = String::new();
			num.push(current);
			while let Some(v) = iter.clone().peekable().peek() {
				if *v == '[' || *v == ']' {
					break;
				}
				num.push(iter.next().unwrap());
			}
			num.split(',').filter_map(|val| val.parse().ok()).map(|v| Item::Value(v)).for_each(|v| current_list.push(v));
		}
	}
	current_list
}

pub fn b(input: &str) -> usize {
	let mut packets: Vec<Vec<Item>> = input.lines()
		.filter(|line| !line.is_empty())
		.map(|line| {
			let mut iter = line.chars();
			parse_array(&mut iter)
		})
		.collect();
	
	let p1 = vec![Item::List(vec![Item::Value(2)])];
	let p2 = vec![Item::List(vec![Item::Value(6)])];

	packets.push(p1.clone());
	packets.push(p2.clone());

	packets.sort_by(|item1, item2| {
		match compare(item1, item2) {
			Some(true) => std::cmp::Ordering::Less,
			Some(false) => std::cmp::Ordering::Greater,
			None => std::cmp::Ordering::Equal,
		}
	});

	let p1_index = packets.iter().enumerate().find(|(_, packet)| packet == &&p1).unwrap().0 + 1;
	let p2_index = packets.iter().enumerate().find(|(_, packet)| packet == &&p2).unwrap().0 + 1;
	p1_index * p2_index
}


