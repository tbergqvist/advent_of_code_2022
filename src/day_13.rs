#[derive(Debug)]
enum Item {
	Value(usize),
	List(Vec<Item>),
}

pub fn a(input: &str) -> usize {
	let pairs: Vec<Item> = input.split("\n\n").flat_map(|group| 
		group.lines()
			.flat_map(|line| {
				let iter = line.chars();
				add_level(iter)
			})
	).collect();

	dbg!(pairs);
	0
}

fn add_level(mut iter: std::str::Chars) -> Vec<Item> {
	let current = iter.next().unwrap();
	if current == '[' {
		let list = add_level(iter);
		vec![Item::List(list)]
	} else {
		let num: String = iter.take_while(|bla| *bla != '[' && *bla != ']').collect();
		num.split(',').filter_map(|val| val.parse().ok()).map(|v| Item::Value(v)).collect()
	}
}

pub fn b(input: &str) -> usize {
	0
}


