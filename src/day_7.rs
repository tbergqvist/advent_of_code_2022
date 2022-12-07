use std::collections::HashMap;

struct Node {
	directories: HashMap<String, Node>,
	files: HashMap<String, usize>
}

impl Node {
	fn new() -> Node {
		Node { directories: HashMap::new(), files: HashMap::new()}
	}
}

pub fn a(input: &str) -> usize {
	let mut root = Node::new();
	read_command(
		input.lines().skip(1).filter(|line| *line != "$ ls" && !line.starts_with("dir")).by_ref(),
		&mut root
	);
	let mut total: usize = 0;
	find_size(&root, &mut total);
	total
}

fn find_size(node: &Node, total: &mut usize) -> usize {
	let size = node.files.values().sum::<usize>() + node.directories.values().map(|d| find_size(d, total)).sum::<usize>();
	if size <= 100000 {
		*total += size;
	}
	size
}

fn read_command<'r, T : Iterator<Item = &'r str>>(lines: &mut T, current_node: &mut Node) {
	while let Some(line) = lines.next()
	{
		if line.starts_with("$ cd ..") {
			return;
		} else if line.starts_with("$ cd") {
			let name = line.split(' ').skip(2).next().unwrap();
			let entry = current_node.directories.entry(name.to_string());
			let current_node = entry.or_insert(Node::new());
			read_command(lines, current_node);
		} else {
			let file_params: Vec<&str> = line.split(' ').collect();
			let size = file_params[0].parse().unwrap();
			current_node.files.entry(file_params[1].to_string()).or_insert(size);
		}
	}
}

fn find_size2(node: &Node, total: &mut Vec<usize>) -> usize {
	let size = node.files.values().sum::<usize>() + node.directories.values().map(|d| find_size2(d, total)).sum::<usize>();
	total.push(size);
	size
}

pub fn b(input: &str) -> usize {
	let mut root = Node::new();
	read_command(
		input.lines().skip(1).filter(|line| *line != "$ ls" && !line.starts_with("dir")).by_ref(),
		&mut root
	);
	let mut bla = 0;
	let total_size = find_size(&root, &mut bla);
	let mut total: Vec<usize> = Vec::new();
	find_size2(&root, &mut total);
	let space_left: i32 = 70000000 - total_size as i32 - 30000000;
	let space_left = space_left.abs() as usize;
	total.into_iter().filter(|t| *t > space_left).min().unwrap()
}