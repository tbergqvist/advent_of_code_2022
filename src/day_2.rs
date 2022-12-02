enum Condition {
  Win,
  Draw,
  Lose
}

#[derive(Clone, Copy)]
enum Move {
  Rock,
  Paper,
  Scissor
}

fn round_to_score((opponent, mine): (Move, Move)) -> i32
{
  match (opponent, mine)
  {
    (Move::Rock, Move::Rock) => 4,
    (Move::Paper, Move::Paper) => 5,
    (Move::Scissor, Move::Scissor) => 6,

    (Move::Rock, Move::Paper) => 8,
    (Move::Rock, Move::Scissor) => 3,

    (Move::Paper, Move::Rock) => 1,
    (Move::Paper, Move::Scissor) => 9,

    (Move::Scissor, Move::Rock) => 7,
    (Move::Scissor, Move::Paper) => 2,
  }
}

pub fn a(input: &str) -> i32 {
  input.lines()
  .map(|i| {
    let split: Vec<Move> = i.split(' ').map(to_move_enum).take(2).collect();
    (split[0], split[1])
  })
  .map(round_to_score)
  .sum()
}

pub fn b(input: &str) -> i32 {
  input.lines()
  .map(|i| {
    let split: Vec<&str> = i.split(' ').take(2).collect();
    (to_move_enum(split[0]), to_condition_enum(split[1]))
  })
  .map(|(opponent, win_condition)| {
    (opponent, match win_condition {
      Condition::Lose => match opponent {
        Move::Rock => Move::Scissor,
        Move::Paper => Move::Rock,
        Move::Scissor => Move::Paper
      },
      Condition::Draw => match opponent {
        Move::Rock => Move::Rock,
        Move::Paper => Move::Paper,
        Move::Scissor => Move::Scissor,
      },
      Condition::Win => match opponent {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissor,
        Move::Scissor => Move::Rock,
      }
    })
  })
  .map(round_to_score)
  .sum()
}

fn to_move_enum(val: &str) -> Move {
  match val {
      "A" | "X" => Move::Rock,
      "B" | "Y" => Move::Paper,
      "C" | "Z" => Move::Scissor,
      _ => panic!()
  }
}

fn to_condition_enum(val: &str) -> Condition {
  match val {
      "X" => Condition::Lose,
      "Y" => Condition::Draw,
      "Z" => Condition::Win,
      _ => panic!()
  }
}