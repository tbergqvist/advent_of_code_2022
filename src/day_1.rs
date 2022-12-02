pub fn a(input: &str) -> i32 {
  input.split("\n\n")
  .map(str::lines)
  .map(|lines| lines
      .map(|l| l.parse::<i32>().unwrap())
      .sum())
  .max()
  .unwrap()
}

pub fn b(input: &str) -> i32 {
  let mut list: Vec<i32> = input.split("\n\n")
  .map(str::lines)
  .map(|lines| lines
      .map(|l| l.parse::<i32>().unwrap())
      .sum())
  .collect();

  list.sort_unstable();
  
  list
    .into_iter()
    .rev()
    .take(3)
    .sum()
}