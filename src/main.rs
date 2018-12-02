use std::fs::File;
use std::io::prelude::*;
use std::collections::{BTreeSet, BTreeMap};

fn read_file(filename: &str) -> String {
  let mut f = File::open(filename).expect("file not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

  contents
}

fn count_letters(s: &str) -> BTreeMap<&str, i32> {
  let mut map = BTreeMap::new();
  let word = s.split("");
  for letter in word {
    map
      .entry(letter)
      .and_modify(|v| *v += 1)
      .or_insert(1);
  }

  map
}

fn has_one_letter_n_times(s: &str, count: i32) -> bool {
  let map = count_letters(s);
  for entry in map {
    if entry.0 == "" {
      continue;
    }
    if entry.1 == count {
      return true;
    }
  }
  false
}

fn has_one_letter_twice(s: &str) -> bool {
  has_one_letter_n_times(s, 2)
}

fn has_one_letter_thrice(s: &str) -> bool {
  has_one_letter_n_times(s, 3)
}

fn day2_problem1() {
    let filename = "./data/input_2-1.txt";

  let input = read_file(filename);
  let serials = input
    .split("\n")
    .filter(|v| *v != "");

  let mut double_letters = 0;
  let mut triple_letters = 0;
  for serial in serials {
    if has_one_letter_twice(serial) {
      double_letters += 1;
    }

    if has_one_letter_thrice(serial) {
      triple_letters += 1;
    }
  }

  println!("Counts are for double {} and triple {}", double_letters, triple_letters);
  println!("Hash sum is {}", double_letters * triple_letters);
}

fn main() {
  day2_problem1();

  println!("Past problems:");
  day1_problem1();
  day1_problem2();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn double_reports_double_letter() {
    let input = "abcddefg";
    assert_eq!(has_one_letter_twice(input), true);
  }

  #[test]
  fn double_does_not_report_triple_letters() {
    let input = "abcdddefg";
    assert_eq!(has_one_letter_twice(input), false);
  }

  #[test]
  fn double_does_not_report_single_letters() {
    let input = "abcdefg";
    assert_eq!(has_one_letter_twice(input), false);
  }

  #[test]
  fn double_does_not_report_quadruple_letters() {
    let input = "abcddddefg";
    assert_eq!(has_one_letter_twice(input), false);
  }

  #[test]
  fn double_reports_only_one_double_letter() {
    let input = "abcddeffg";
    assert_eq!(has_one_letter_twice(input), true);
  }

    #[test]
  fn triple_reports_triple_letter() {
    let input = "abcdddefg";
    assert_eq!(has_one_letter_thrice(input), true);
  }

  #[test]
  fn triple_does_not_report_double_letters() {
    let input = "abcddefg";
    assert_eq!(has_one_letter_thrice(input), false);
  }

  #[test]
  fn triple_does_not_report_single_letters() {
    let input = "abcdefg";
    assert_eq!(has_one_letter_thrice(input), false);
  }

  #[test]
  fn triple_does_not_report_quadruple_letters() {
    let input = "abcddddefg";
    assert_eq!(has_one_letter_thrice(input), false);
  }

  #[test]
  fn triple_reports_only_one_triple_letter() {
    let input = "abcdddefffg";
    assert_eq!(has_one_letter_thrice(input), true);
  }
}

fn day1_problem1() {
  let filename = "./data/input_1-1.txt";

  let input = read_file(filename);
  let result: i32 = input
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| v.parse::<i32>().unwrap())
    .sum();

  println!("Result of 1-1: {}", result);
}

fn day1_problem2() {
  let filename = "./data/input_1-1.txt";

  let input = read_file(filename);
  let numbers = input
    .split("\n")
    .filter(|v| *v != "")
    .map(|v| v.parse::<i32>().unwrap())
    .collect::<Vec<_>>();

  let mut visited_frequencies = BTreeSet::new();

  let mut result = 0;
  let mut index = 0;
  while !visited_frequencies.contains(&result) {
    visited_frequencies.insert(result);
    result += numbers[index];
    index = (index + 1) % numbers.len();
  }

  println!("First result visited twice: {}", result);
  println!("Number of results generated: {}", visited_frequencies.len());
}