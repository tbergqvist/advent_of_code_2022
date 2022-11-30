use std::fs;

mod day_1;

fn main() {
  println!("Solving");

  let day1_input = fs::read_to_string("./inputs/1.txt").unwrap();
  println!("1a:{}", day_1::a(&day1_input));
  println!("1b:{}", day_1::b(&day1_input));
  
  println!("Done");
}
