use std::str::FromStr;
use super::board::Board;
use super::super::utils::Error;

fn run_simulation(minutes: usize) -> Result<Vec<usize>, Error> {
  let input = include_str!("./data/input.txt");
  let mut board = Board::from_str(input)?;

  board.debug();

  let mut checksums = vec![];

  for i in 0..minutes {
    if i % 1_000 == 0 {
      println!("Round {}", i);
    }
    checksums.push(board.checksum());
    board.tick();
  }
  board.debug();

  Ok(checksums)
}

pub fn problem1() -> Result<usize, Error> {
  let mut checksums = run_simulation(10)?;

  let result = match checksums.pop() {
    Some(v) => v,
    None => Err(Error::new("No checksums found"))?
  };

  Ok(result)
}

fn find_cycle(checksums: &Vec<usize>) -> Option<usize> {
  if checksums.len() < 1000 {
    return None;
  }

  let first = 800;
  let find = checksums[first];
  for (i, c) in checksums.iter().skip(first + 1).enumerate() {
    if *c == find {
      return Some(i + 1);
    }
  }

  None
}

pub fn problem2() -> Result<usize, Error> {
  // takes too long
  // let result = run_simulation(1_000_000_000)?;

  let simulated = 10_000;
  let checksums = run_simulation(simulated)?;

  let iterations = 1_000_000_000;
  let cycle = find_cycle(&checksums).unwrap();

  let offset = (iterations - 1_000) % cycle;
  let result = checksums[1000 + offset];

  println!("Result: {}", result);

  Ok(result)
}
