use std::fs;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
  println!("Solving");

  let day1_input = fs::read_to_string("./inputs/1.txt").unwrap();
  println!("1a:{}", day_1::a(&day1_input));
  println!("1b:{}", day_1::b(&day1_input));
  
  let day2_input = fs::read_to_string("./inputs/2.txt").unwrap();
  println!("2a:{}", day_2::a(&day2_input));
  println!("2b:{}", day_2::b(&day2_input));
  
  let day3_input = fs::read_to_string("./inputs/3.txt").unwrap();
  println!("3a:{}", day_3::a(&day3_input));
  println!("3b:{}", day_3::b(&day3_input));
  
  let day4_input = fs::read_to_string("./inputs/4.txt").unwrap();
  println!("4a:{}", day_4::a(&day4_input));
  println!("4b:{}", day_4::b(&day4_input));

  println!("Done");
}
