use std::str::FromStr;
use super::board::Board;
use super::super::utils::Error;

fn run_simulation(minutes: usize) -> Result<usize, Error> {
  let input = include_str!("./data/input.txt");
  let mut board = Board::from_str(input)?;

  board.debug();

  for i in 0..minutes {
    if i % 1_000_000 == 0 {
      println!("Round {}", i);
    }
    board.tick();
    // board.debug();
  }
  board.debug();

  Ok(board.checksum())
}

pub fn problem1() -> Result<usize, Error> {
  let result = run_simulation(10)?;
  println!("Result: {}", result);

  Ok(result)
}

pub fn problem2() -> Result<usize, Error> {
  let result = run_simulation(1_000_000_000)?;
  println!("Result: {}", result);

  Ok(result)
}
