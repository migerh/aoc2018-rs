use std::collections::HashSet;
use super::super::utils::Error;
use super::parser::parse;
use super::analysis::{Position, shortest_path};

fn run_problem1(input: &str) -> Result<usize, Error> {
  let mut nodes = parse(input);
  let mut map = HashSet::new();
  let mut pos: Position = (0, 0);
  if let Some(root) = nodes.pop() {
    let result = shortest_path(root, &mut pos, &mut map, 0);
    println!("Result: {}", result);
    Ok(result)
  } else {
    Err(Error::new("No root node found."))
  }
}

pub fn problem1() -> Result<usize, Error> {
  let input = include_str!("./data/input.txt");
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