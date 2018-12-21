use std::collections::HashMap;
use super::super::utils::Error;
use super::parser::parse;
use super::analysis::{Position, count_rooms};

fn run_problem1(input: &str) -> Result<usize, Error> {
  let nodes = parse(input);

  // println!("Nodes: {:?}", nodes);

  let mut map = HashMap::new();
  let mut pos: Position = (0, 0);
  map.insert(pos, 0);
  let mut rooms = 0;
  let distance = count_rooms(&nodes, &mut pos, &mut map, 0, &mut rooms);

  let number_of_rooms = map.iter().filter(|(_, dist)| **dist >= 1000).count();

  println!("Distance: {}", distance);
  println!("Number of Rooms: {}", number_of_rooms);

  Ok(distance)
}

pub fn problem1() -> Result<usize, Error> {
  let input = include_str!("./data/input.txt");
  // let input = "^ENWWW(NEEE|SSE(EE|N))$";
  // let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
  // let input = "^WNE$";

  // println!("Input: {}", input);
  run_problem1(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_example1() {
    assert_eq!(run_problem1("^WNE$").unwrap(), 3);
  }

  #[test]
  fn check_example2() {
    assert_eq!(run_problem1("^ENWWW(NEEE|SSE(EE|N))$").unwrap(), 10);
  }

  #[test]
  fn check_example3() {
    assert_eq!(run_problem1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").unwrap(), 18);
  }

  #[test]
  fn check_example4() {
    assert_eq!(run_problem1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$").unwrap(), 23);
  }

  #[test]
  fn check_example5() {
    assert_eq!(run_problem1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$").unwrap(), 31);
  }
}