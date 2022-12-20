#[derive(Debug)]
struct Sensor {
	x: i64,
	y: i64,
	beacon_x: i64,
	beacon_y: i64,
	distance: i64,

}

fn find_pos(input: &str, pattern: &str) -> i64 {
	let x_start = input.find(pattern).unwrap() + 2;
	let x_end = input[x_start..].find(",").or_else(||input[x_start..].find(":")).map(|b| b + x_start);
	let input = if let Some(x_end) = x_end {
		&input[x_start..x_end]
	} else {
		&input[x_start..]
	};
	input.parse().unwrap()
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
	input.lines().map(|line| {
		let sensor_x = find_pos(&line[..36], "x=");
		let sensor_y = find_pos(&line[..36], "y=");
		let beacon_x = find_pos(&line[36..], "x=");
		let beacon_y = find_pos(&line[36..], "y=");

		Sensor {
			y: sensor_y,
			x: sensor_x,
			beacon_x,
			beacon_y,
			distance: (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs()
		}
	}).collect()
}

fn is_in_range(sensors: &Vec<Sensor>, x: i64, y: i64) -> Option<&Sensor> {
	sensors.iter().find(|sensor| {
		(x - sensor.x).abs() + (y - sensor.y).abs() <= sensor.distance
	})
}

fn is_beacon(sensors: &Vec<Sensor>, x: i64, y: i64) -> bool {
	sensors.iter().any(|sensor| {
		x == sensor.beacon_x && y == sensor.beacon_y
	})
}

pub fn a(input: &str) -> usize {
	let sensors = parse_sensors(input);
	let space_min = sensors.iter().map(|s| s.x - s.distance).min().unwrap();
	let space_max = sensors.iter().map(|s| s.x + s.distance).max().unwrap();

	let y_pos = 2000000;

	(space_min..=space_max)
		.filter(|i| !is_beacon(&sensors, *i, y_pos))
		.filter(|i| is_in_range(&sensors, *i, y_pos).is_some())
	.count()
}

pub fn b(input: &str) -> i64 {
	let sensors = parse_sensors(input);
	let space_max = 4000000;

	for y in 0..space_max {
		let mut x = 0;
		while x < space_max {
			if let Some(sensor) = is_in_range(&sensors, x, y) {
				x = sensor.distance - (sensor.y - y).abs() + sensor.x + 1;
			} else {
				return x * 4000000 + y
			}
		}
	}

	0
}


