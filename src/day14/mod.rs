use super::utils::{Error, ParseError};
use std::num::ParseIntError;
use std::vec::Vec;

fn initialize() -> Result<Vec<u8>, ParseError> {
  let input = "37";

  let result = input
    .chars()
    .map(|c| c.to_string().parse::<u8>())
    .collect::<Result<Vec<u8>, ParseIntError>>()?;

  Ok(result)
}

fn concat(numbers: &[u8]) -> String {
  numbers.iter().map(|n| n.to_string()).collect::<String>()
}

fn compare(v1: &[u8], v2: &[u8]) -> bool {
  if v1.len() != v2.len() {
    return false;
  }

  let l = v1.len();
  for i in 0..l {
    if v1[i] != v2[i] {
      return false;
    }
  }
  true
}

pub fn problem1() -> Result<(), Error> {
  let mut scores = initialize()?;
  let pattern: Vec<u8> = vec![2, 0, 9, 2, 3, 1];

  let mut current: (usize, usize) = (0, 1);
  let mut solution_2 = 0;
  for i in 0..100_000_000_000usize {
    if i % 1_000_000 == 0 {
      println!("Iteration {}", i);
    }
    let new_score = scores[current.0] + scores[current.1];
    if new_score > 9 {
      let next_1 = new_score / 10;
      let next_2 = new_score % 10;
      scores.push(next_1);
      scores.push(next_2);
    } else {
      scores.push(new_score);
    }

    current.0 = (current.0 + 1 + scores[current.0] as usize) % scores.len();
    current.1 = (current.1 + 1 + scores[current.1] as usize) % scores.len();

    let l = scores.len();
    let pl = pattern.len();
    if l > pl + 2 {
      let first = &scores[l-pl-1..l-1];
      if compare(first, &pattern[..]) {
        solution_2 = l - pl - 1;
        break;
      }

      let second = &scores[l-pl..l];
      if compare(second, &pattern[..]) {
        solution_2 = l - pl - 1;
        break;
      }
    }
  }

  println!("Solution for problem 1: {}", concat(&scores[209231..209241]));
  println!("Solution for problem 2: {}", solution_2);

  Ok(())
}