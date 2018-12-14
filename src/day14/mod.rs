use super::utils::{Error, ParseError};
use std::num::ParseIntError;
use std::cmp::{max, min};
use std::vec::Vec;

fn initialize() -> Result<Vec<u8>, ParseError> {
  let input = "37";

  let result = input
    .chars()
    .map(|c| c.to_string().parse::<u8>())
    .collect::<Result<Vec<u8>, ParseIntError>>()?;

  Ok(result)
}

fn concat(numbers: &Vec<u8>) -> String {
  numbers.iter().map(|n| n.to_string()).collect::<String>()
}

fn check(next: u8, pos: usize, pattern: &Vec<u8>) -> (usize, bool) {
  let mut new_pos = 0;
  if pattern[pos] == next {
    new_pos = pos + 1;
  } else {
    new_pos = 0;
    if pattern[new_pos] == next {
      new_pos = 1;
    }
  }
  if new_pos == pattern.len() {
    (new_pos, true)
  } else {
    (new_pos, false)
  }
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
  // let pattern: Vec<u8> = vec![1, 8, 4, 5, 3, 2];
  let pattern_str = "209231";
  println!("{:?}", scores);
  println!("{:?}", pattern);

  let mut current: (usize, usize) = (0, 1);
  let mut next_check: usize = 0;
  let mut collect = false;
  let mut result = vec![];
  // for i in 0..100_000_000_000usize {
  for i in 0..15 {
    if i % 1_000_000 == 0 {
      println!("Iteration {}", i);
    }
    let new_score = scores[current.0] + scores[current.1];
    if new_score > 9 {
      let next_1 = new_score / 10;
      if collect {
        result.push(next_1);
      }
      let (new_pos, coll) = check(next_1, next_check, &pattern);
      next_check = new_pos;
      collect = coll;
      // if pattern[next_check] == next_1 {
      //   next_check += 1;
      // } else {
      //   next_check = 0;
      //   if pattern[next_check] == next_1 {
      //     next_check += 1;
      //   }
      // }
      // if next_check == pattern.len() as u8 {
      //   collect = true;
      // }

      let next_2 = new_score / 10;
      if collect {
        result.push(next_2);
      }
      let (new_pos, coll) = check(next_2, next_check, &pattern);
      next_check = new_pos;
      collect = coll;
      // if pattern[next_check] == next_2 {
      //   next_check += 1;
      // } else {
      //   next_check = 0;
      //   if pattern[next_check] == next_2 {
      //     next_check += 1;
      //   }
      // }
      // if next_check == pattern.len() as u8 {
      //   collect = true;
      // }

      scores.push(next_1);
      scores.push(next_2);
    } else {
      let next = new_score;
      if collect {
        result.push(next);
      }
      let (new_pos, coll) = check(next, next_check, &pattern);
      next_check = new_pos;
      collect = coll;
      // if pattern[next_check] == next {
      //   next_check += 1;
      // } else {
      //   next_check = 0;
      //   if pattern[next_check] == next {
      //     next_check += 1;
      //   }
      // }
      scores.push(new_score);
    }

    current.0 = (current.0 + 1 + scores[current.0] as usize) % scores.len();
    current.1 = (current.1 + 1 + scores[current.1] as usize) % scores.len();

    // if next_check > 0 {
    //   println!("Found something: {}", next_check);
    // }

    if result.len() > 10 {
      println!("{:?}", result);
      break;
    }

    let l = scores.len();
    let pl = pattern.len();
    // println!("Pattern: {:?}", &pattern[..]);
    println!("Scores: {:?}", &scores[..]);
    if l > pl + 2 {
      let first = &scores[l-pl-1..l-1];
      // println!("First: {:?}", first);
      if compare(first, &pattern[..]) {
        println!("(first) Found something at {}", l - pl - 1);
        break;
      }

      let second = &scores[l-pl..l];
      // println!("Second: {:?}", second);
      if compare(second, &pattern[..]) {
        println!("(second) Found something at {}", l - pl);
        break;
      }
    }

    // let score_str = concat(&scores);
    // // let from = max(i - 10, 0) as usize;
    // let sub = min(score_str.len(), 20);
    // let from = max(score_str.len() - sub, 0);
    // let to = score_str.len();
    // let position = score_str[from..to].find(pattern_str);

    // match position {
    //   Some(v) => println!("Found at {}", v),
    //   None => {}
    // }
  }

  Ok(())
}