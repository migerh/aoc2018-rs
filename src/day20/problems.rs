use std::collections::HashMap;
use super::super::utils::Error;
use super::parser::parse;
use super::analysis::{Position, count_rooms};

fn run(input: &str) -> Result<(usize, usize), Error> {
  let nodes = parse(input);

  let mut map = HashMap::new();
  let mut pos: Position = (0, 0);
  map.insert(pos, 0);
  let mut rooms = 0;
  let distance = count_rooms(&nodes, &mut pos, &mut map, 0, &mut rooms);

  let number_of_rooms = map.iter().filter(|(_, dist)| **dist >= 1000).count();

  println!("Distance: {}", distance);
  println!("Number of Rooms: {}", number_of_rooms);

  Ok((distance, number_of_rooms))
}

pub fn problems() -> Result<(usize, usize), Error> {
  let input = include_str!("./data/input.txt");
  // let input = "^ENWWW(NEEE|SSE(EE|N))$";
  // let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
  // let input = "^WNE$";

  run(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_example1() {
    assert_eq!(run("^WNE$").unwrap().0, 3);
  }

  #[test]
  fn check_example2() {
    assert_eq!(run("^ENWWW(NEEE|SSE(EE|N))$").unwrap().0, 10);
  }

  #[test]
  fn check_example3() {
    assert_eq!(run("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").unwrap().0, 18);
  }

  #[test]
  fn check_example4() {
    assert_eq!(run("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$").unwrap().0, 23);
  }

  #[test]
  fn check_example5() {
    assert_eq!(run("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$").unwrap().0, 31);
  }
}