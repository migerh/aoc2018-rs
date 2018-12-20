use std::collections::HashSet;
use super::super::utils::Error;
use super::parser::parse;
use super::analysis::{Position, shortest_path};

fn run_problem1(input: &str) -> Result<usize, Error> {
  let nodes = parse(input);

  // println!("Nodes: {:?}", nodes);
  // println!("Found {} root nodes", nodes.len());

  let mut map = HashSet::new();
  let mut pos: Position = (0, 0);
  let mut distance = 0;
  for root in nodes {
    // println!("Looking at node {:?}", root);
    let result = shortest_path(root, &mut pos, &mut map, 0);
    // println!("Intermediate result: {}", result);
    distance += result;
  }

  println!("Result: {}", distance);
  Ok(distance)
}

pub fn problem1() -> Result<usize, Error> {
  // let input = include_str!("./data/input.txt");
  // let input = "^ENWWW(NEEE|SSE(EE|N))$";
  let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
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