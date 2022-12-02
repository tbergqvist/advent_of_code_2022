use std::fs;

mod day_1;
mod day_2;

fn main() {
  println!("Solving");

  let day1_input = fs::read_to_string("./inputs/1.txt").unwrap();
  println!("1a:{}", day_1::a(&day1_input));
  println!("1b:{}", day_1::b(&day1_input));
  
  let day2_input = fs::read_to_string("./inputs/2.txt").unwrap();
  println!("2a:{}", day_2::a(&day2_input));
  println!("2b:{}", day_2::b(&day2_input));

  println!("Done");
}
