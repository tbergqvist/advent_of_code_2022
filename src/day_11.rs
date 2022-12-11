#[derive(Debug)]
enum Operation {
	Add(Option<i64>),
	Multiply(Option<i64>)
}

impl Operation {
	fn do_op(&self, worry: i64) -> i64 {
		match self {
			Self::Add(val) => worry + val.unwrap_or(worry),
			Self::Multiply(val) => worry * val.unwrap_or(worry),
		}
	}
}

#[derive(Debug)]
struct Monkey {
	items: Vec<i64>,
	operation: Operation,
	test_divider: i64,
	monkey_1: usize,
	monkey_2: usize,
	total_throws: usize
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
	input.split("\nMonkey")
	.map(|monkey| {
		let mut lines = monkey.lines().skip(1);
		let starting_items_line: &str = lines.next().unwrap();
		let starting_items = starting_items_line[18..].split(", ").filter_map(|v| v.parse().ok()).collect();
		
		let operation_line: &str = lines.next().unwrap();
		let mut op_split = operation_line[23..].split(' ').collect::<Vec<&str>>().into_iter().rev();
		let num = op_split.next().and_then(|v| v.parse().ok());
		let op = op_split.next().map(|o| 
			if o == "+" {Operation::Add(num)} else {Operation::Multiply(num)}
		).unwrap();
		
		let test_line = lines.next().unwrap();
		let test_divider = test_line[21..].parse().unwrap();
		
		let monkey_1_line = lines.next().unwrap();
		let monkey_1 = monkey_1_line[29..].parse().unwrap();

		let monkey_2_line = lines.next().unwrap();
		let monkey_2 = monkey_2_line[30..].parse().unwrap();

		Monkey {
			items: starting_items,
			operation: op,
			test_divider,
			monkey_1,
			monkey_2,
			total_throws: 0
		}
	}).collect()
}

pub fn a(input: &str) -> usize {
	let mut monkeys: Vec<Monkey> = parse_monkeys(input);

	for _ in 0..20 {
		for i in 0..monkeys.len() {
			let monkey = &monkeys[i];
			let items_to_throw: Vec<(i64, usize)> = monkey.items.iter().map(|item| {
				let new_val = monkey.operation.do_op(*item);
				let new_val = new_val / 3;
				(new_val, if new_val % monkey.test_divider == 0 {
					monkey.monkey_1
				} else {
					monkey.monkey_2
				})
			}).collect();

			items_to_throw.into_iter().for_each(|(item, money_index)| {
				monkeys[money_index].items.push(item);
			});

			let monkey = &mut monkeys[i];
			monkey.total_throws += monkey.items.len();
			monkey.items = Vec::new();
		}
	};
	
	let mut throws: Vec<usize> = monkeys.into_iter().map(|monkey|monkey.total_throws).collect();
	throws.sort();
	throws.into_iter().rev().take(2).reduce(|v, v2| v * v2).unwrap()
}

pub fn b(input: &str) -> usize {
	let mut monkeys: Vec<Monkey> = parse_monkeys(input);

	let cool_divider = monkeys.iter().fold(1, |i, i2| i * i2.test_divider);

	for _ in 0..10000 {
		for i in 0..monkeys.len() {
			let monkey = &monkeys[i];
			let items_to_throw: Vec<(i64, usize)> = monkey.items.iter().map(|item| {
				let new_val = monkey.operation.do_op(*item);
				let new_val = new_val % cool_divider;
				(new_val, if new_val % monkey.test_divider == 0 {
					monkey.monkey_1
				} else {
					monkey.monkey_2
				})
			}).collect();

			items_to_throw.into_iter().for_each(|(item, money_index)| {
				monkeys[money_index].items.push(item);
			});

			let monkey = &mut monkeys[i];
			monkey.total_throws += monkey.items.len();
			monkey.items = Vec::new();
		}
	};
	let mut throws: Vec<usize> = monkeys.into_iter().map(|monkey|monkey.total_throws).collect();
	throws.sort();
	throws.into_iter().rev().take(2).reduce(|v, v2| v * v2).unwrap()
}