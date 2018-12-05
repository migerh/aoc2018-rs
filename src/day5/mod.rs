fn same_letter(a: char, b: char) -> bool {
  a.to_ascii_lowercase() == b.to_ascii_lowercase()
}

fn different_cases(a: char, b: char) -> bool {
  (a.is_lowercase() && b.is_uppercase()) || (a.is_uppercase() && b.is_lowercase())
}

fn letters_cancel(a: char, b: char) -> bool {
  same_letter(a, b) && different_cases(a, b)
}

fn reduce(s: String) -> String {
  let chars = s.chars().collect::<Vec<_>>();

  let mut result = "".to_string();
  let last = chars.len() - 1;
  let mut last_one_matched = false;
  for i in 0..last {
    if last_one_matched {
      last_one_matched = false;
      continue;
    }
    let chr = chars[i];
    let peek = chars[i + 1];

    last_one_matched = letters_cancel(chr, peek);
    if !last_one_matched {
      result.push(chr);
    }
  }

  if !last_one_matched {
    let chr = chars[last - 1];
    let last = chars[last];
    if !letters_cancel(chr, last) {
      result.push(last);
    }
  }

  result
}

pub fn reduce_repeatedly(input: String) -> String {
  let mut old_len = input.len();
  let mut polymer = reduce(input.to_string());
  while polymer.len() < old_len {
    old_len = polymer.len();
    polymer = reduce(polymer);
  }

  polymer
}

pub fn problem1() -> usize {
  let mut input = include_str!("./data/input.txt").to_string();
  input.retain(|v| v.is_alphabetic());
  let polymer = reduce_repeatedly(input);

  println!("Result: {}", polymer.len());

  polymer.len()
}

pub fn problem2() -> usize {
  let mut input = include_str!("./data/input.txt").to_string();
  input.retain(|v| v.is_alphabetic());
  let input = input;
  let start: u8 = 65;
  let end: u8 = 91;

  let mut smallest_char = 'a';
  let mut smallest_len = input.len();
  for c in start..end {
    let chr = c as char;
    let mut filtered = input.clone();
    filtered.retain(|v| v != chr && v != chr.to_ascii_lowercase());
    let reduced_polymer = reduce_repeatedly(filtered);
    let len = reduced_polymer.len();

    if len < smallest_len {
      smallest_len = len;
      smallest_char = chr;
    }
  }

  println!("Smallest polymer of length {} produced by removing char {}", smallest_len, smallest_char);

  smallest_len
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn reduce_reduces() {
    let input = "aaAb".to_string();
    assert_eq!(reduce(input), "ab".to_string())
  }

  #[test]
  fn reduce_reduces_longer_strings() {
    let input = "aaAaAbc".to_string();
    assert_eq!(reduce(input), "abc".to_string())
  }

  #[test]
  fn reduce_reduces_even_longer_strings() {
    let input = "aaAaAbBcCdeAan".to_string();
    assert_eq!(reduce(input), "aden".to_string())
  }

  #[test]
  fn reduce_repeatedly_simple_case() {
    let input = "aaaAAcAAab".to_string();
    assert_eq!(reduce_repeatedly(input), "acAb".to_string())
  }

  #[test]
  fn problem1_returns_correct_result() {
    assert_eq!(problem1(), 9078);
  }
}