use std::option::Option;
use std::collections::BTreeMap;

fn number_of_different_letters(a: &str, b: &str) -> i32 {
  let mut diffs = 0;

  for ch in a.chars().zip(b.chars()) {
    if ch.0 != ch.1 {
      diffs += 1;
    }
  }

  diffs
}

fn find_similar<'a>(list: &Vec<&'a str>, s: &'a str) -> Option<&'a str> {
  for l in list {
    if number_of_different_letters(l, s) == 1 {
      return Some(l);
    }
  }

  None
}

pub fn problem2() {
  let input = include_str!("./data/input_2-1.txt");
  let serials = input
    .split("\n")
    .filter(|v| *v != "")
    .collect();

  for serial in &serials {
    match find_similar(&serials, serial) {
      Some(v) => println!("{} is close to {}", serial, v),
      None => (),
    }
  }
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

pub fn problem1() {
  let input = include_str!("./data/input_2-1.txt");
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
