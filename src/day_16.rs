use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Valve {
	name: String,
	pressure: i32,
	neighbors: Vec<String>,
}

#[derive(Debug, Clone)]
struct Node {
	name: String,
	cost: i32,
}

#[derive(Debug, Clone)]
struct RealValve {
	name: String,
	pressure: i32,
	neighbors: Vec<Node>,
	open: bool,
}

type Valves = HashMap<String, RealValve>;

fn find_cost(valves: &Vec<Valve>, v1: &Valve, v2: &Valve, current_cost: i32, searched: &mut HashMap<String, i32>) -> Option<i32> {
	let key: String = v1.name.clone() + &v2.name;
	
	if searched.contains_key(&key) {
		return Some(searched[&key]);
	}

	if current_cost > valves.len() as i32 {
		return None;
	}

	let cost = if v2.neighbors.iter().any(|n| n == &v1.name) {
		Some(1)
	} else {
		v2.neighbors.iter().filter_map(|n| {
			let node = valves.iter().find(|i| &i.name == n).unwrap();
			find_cost(valves, v1, node, current_cost + 1, searched).map(|v| v + 1)
		}).min()
	};

	if cost.is_some() && !searched.contains_key(&key) {
		searched.insert(key, cost.unwrap());
	}
	cost
}

fn parse_valves(input: &str) -> Valves {
	//parse lines
	let valves: Vec<Valve> = input.lines().map(|line| {
		let name = line[6..8].to_string();
		let pressure_end = line[23..].find(';').unwrap() + 23;

		let pressure = line[23..pressure_end].parse().unwrap();
		let list_start = line[46..].find(' ').unwrap() + 46;
		let neighbors = line[list_start..].split(",").map(|s| s.trim().to_string()).collect();
		Valve {
			name,
			pressure,
			neighbors
		}	
	}).collect();

	let mut searched = HashMap::new();
	// calc cost toward each valve
	let valves: Vec<RealValve> = valves.iter().map(|valve| {
			RealValve {
				name: valve.name.clone(),
				pressure: valve.pressure,
				neighbors: valves.iter().filter(|n| n.name != valve.name).map(|neighbor| {
					let cost = find_cost(&valves, valve, neighbor, 1, &mut searched);
					Node {
						cost: cost.unwrap(),
						name: neighbor.name.clone()
					}
				}).collect(),
				open: false
			}
	}).collect();

	//remove valves that give 0
	let zero_valves: HashSet<String> = valves.iter().filter(|v| v.pressure == 0 && v.name != "AA").map(|v| v.name.clone()).collect();
	valves.into_iter()
		.filter(|v| !zero_valves.contains(&v.name))
		.map(|v| {
			(
				v.name.clone(),
				RealValve {
					neighbors: v.neighbors.into_iter().filter(|v| !zero_valves.contains(&v.name) && v.name != "AA").collect(),
					..v
				}
			)
		})
		.collect()
}

fn visit(valves: Valves, valve_name: &String, time_left: i32, current_pressure: i32, total_pressure: i32) -> i32 {
	let opened_valves = {
		let mut clone = valves.clone();
		let mut valve = clone.get_mut(valve_name).unwrap();
		valve.open = true;
		clone
	};

	let current_valve = &valves[valve_name];
	let time_left = time_left - 1;
	if time_left <= 0 {
		return total_pressure + current_pressure;
	}
	let total_pressure = total_pressure + current_pressure;
	let current_pressure = current_pressure + current_valve.pressure;

	current_valve.neighbors.iter()
		.filter(|n| !valves[&n.name].open)
		.map(|n| {
			let new_time_left = time_left - n.cost;
			if new_time_left <= 0 {
				total_pressure + current_pressure * time_left
			} else {
				visit(opened_valves.clone(), &n.name, new_time_left, current_pressure, total_pressure + current_pressure * n.cost)
			}
		}
		)
		.max()
		.unwrap_or(total_pressure + current_pressure * time_left)
}

pub fn a(input: &str) -> i32 {
	let valves = parse_valves(input);
	visit(valves, &"AA".to_string(), 31, 0, 0)
}

pub fn b(input: &str) -> i64 {
	0
}


