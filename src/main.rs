use std::fs;
use paste::paste;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;

macro_rules! run_days {
	( $( $x:expr), + ) => {
		$(
			{
				paste!{
					let input = fs::read_to_string(
						concat!("./inputs/", stringify!($x), ".txt")
					).unwrap();

					println!(concat!($x, "a:{}"), [<day _ $x>]::a(&input));
					println!(concat!($x, "b:{}"), [<day _ $x>]::b(&input));
				}
			}
		)*
	}
}

fn main() {
	run_days!(1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17);
}
